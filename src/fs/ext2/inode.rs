/*
 * Copyright (C) 2018-2019 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see https://www.gnu.org/licenses.
 */

use crate::fs::ext2::superblock::Superblock;
use crate::fs::ext2::gd::GDTable;
use crate::fs::ext2::InodeMode;
use crate::disk::sata::read_disk;
use crate::errors::*;
use ::read_num_bytes;

use core::ptr::copy_nonoverlapping;
use core::str::from_utf8;
use alloc::prelude::v1::{ToString, String, Vec};

#[derive(Debug, Copy, Clone)]
pub struct Inode
{
    pub i_mode: u16,    // File mode
    pub i_uid: u16,     // Owner Uid
    pub i_size: u32,    // Size in bytes
    pub i_atime: u32,    // Access time
    pub i_ctime: u32,    // Creation time
    pub i_mtime: u32,    // Modification time
    pub i_dtime: u32,    // Deletion Time
    pub i_gid: u16,     // Group Id
    pub i_links_count: u16,    // Links count
    pub i_blocks: u32,    // Number of blocks allocated for the file
    pub i_flags: u32,    // File flags
    pub _osd1: u32,     // OS Dependent #1
    pub i_block: [u32; 15],    // Pointers to blocks
    pub i_version: u32,    // File version (for NFS)
    pub i_file_acl: u32,    // File ACL
    pub i_dir_acl: u32,    // Directory ACL / Extended File Size
    pub i_faddr: u32,    // Fragment address
    pub _osd2: [u32; 3],    // OS Dependent #2 (Typically fragment info)
}

pub struct DirEntry {
    _inode: u32,
    _len: u16,
    _name_len: u8,
    _file_type: u8,
    _name: String,
}

impl Inode {
    /// Read an inode
    pub fn new(partition_start: u64, i_num: u32, block_size: u32, sb: Superblock, gdt: &GDTable) -> Self {
        // TODO: Use plain for safe
        let gr_num = (i_num - 1) / sb.data.s_inodes_per_group;

        let index = (i_num - 1) % sb.data.s_inodes_per_group;

        let inode_table_start = gdt.0[gr_num as usize].bg_inode_table as u64 * block_size as u64;

        let offset: u64 = inode_table_start + index as u64 * sb.ext.s_inode_size as u64;

        let value = &read_disk(partition_start + offset as u64,
                               partition_start + offset as u64 + sb.ext.s_inode_size as u64)
                               .expect("cannot read");
        Self::from_slice(value)
    }

    pub fn max_block(&self) -> u32 {
        // TODO: Use the Superblock block size
        let n_blocks = (self.i_size as u64 + 4096 as u64 - 1) / 4096 as u64;
        if n_blocks > core::u32::MAX as u64 {
                core::u32::MAX
        }
        else {
                n_blocks as u32
        }
    }

    pub fn from_slice(data: &[u8]) -> Self{
        Self {
            i_mode: read_num_bytes!(u16, &data[0..2]),
            i_uid: read_num_bytes!(u16, &data[2..4]),
            i_size: read_num_bytes!(u32, &data[4..8]),
            i_atime: read_num_bytes!(u32, &data[8..12]),
            i_ctime: read_num_bytes!(u32, &data[12..16]),
            i_mtime: read_num_bytes!(u32, &data[16..20]),
            i_dtime: read_num_bytes!(u32, &data[20..24]),
            i_gid: read_num_bytes!(u16, &data[24..26]),
            i_links_count: read_num_bytes!(u16, &data[26..28]),
            i_blocks: read_num_bytes!(u32, &data[28..32]),
            i_flags: read_num_bytes!(u32, &data[32..36]),
            _osd1: read_num_bytes!(u32, &data[36..40]),
            i_block: unsafe {*(&data[40..100] as *const _ as *const [u32; 15])},
            i_version: read_num_bytes!(u32, &data[100..104]),
            i_file_acl: read_num_bytes!(u32, &data[104..108]),
            i_dir_acl: read_num_bytes!(u32, &data[108..112]),
            i_faddr: read_num_bytes!(u32, &data[112..116]),
            _osd2: unsafe {*(&data[116..128] as *const _ as *const [u32; 3])},
        }
    }

    /// Verify if an inode is a directory
    pub fn is_directory(&self) -> bool {
        if self.i_mode & InodeMode::Ext2SIfdir as u16 != 0 {true} else {false}
    }

    pub fn get_dir_entries(&self) -> Vec<DirEntry>{
        let files = Vec::new();
        files
    }

    pub fn read(&self, partition_start: u64) -> Result<String> {
        let full_size = self.i_size;
        if self.i_block[12] != 0 {
            // TODO: Indirect blocks
            Ok("".to_string())
        }
        else if self.i_block[13] != 0 {
            // TODO: Bi-indirect blocks
            Ok("".to_string())
        }
        else if self.i_block[14] != 0 {
            // TODO: Tri-indirect blocks
            Ok("".to_string())
        }
        else {
            let mut buf = Vec::new();
            for i in 0..12 {
                if self.i_block[i] == 0 {
                    break;
                }
                let block = partition_start + 4096 * self.i_block[i] as u64;

                buf = read_disk(block, block + full_size as u64).expect("cannot read disk");
            }
            if buf.as_slice() == [] {
                return Err(Error::new(ENXIO));
            }
            else {
                let content = from_utf8(&buf)
                    .expect("cannot create an utf-8 string").to_string();
                return Ok(content);
            }
        }
    }
}