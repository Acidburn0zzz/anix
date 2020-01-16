/*
 * Copyright (C) 2018-2020 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
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
use alloc::prelude::v1::{String, Vec};

use crate::errors::{Error, Result};
use super::number::*;

// Helpers (the 0, 1, 2, 3, 4, ... numbers in the function name is the number of arguments)
pub unsafe fn syscall0(mut a: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a)
        : "rcx", "r11", "memory"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall1(mut a: usize, b: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b)
        : "rcx", "r11", "memory"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall2(mut a: usize, b: usize, c: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b), "{rsi}"(c)
        : "rcx", "r11", "memory"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall3(mut a: usize, b: usize, c: usize, d: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b), "{rsi}"(c), "{rdx}"(d)
        : "rcx", "r11", "memory"
        : "intel", "volatile");

    Error::demux(a)
}

// Functions
pub fn exit(code: usize) -> usize {
    unsafe {
        syscall1(SYS_EXIT, code).expect("cannot exit")
    }
}

use crate::time::DateTime;
pub fn date() -> DateTime {
    unsafe {
        let reference: &[DateTime] = &[DateTime::default()];
        syscall2(SYS_TIME, reference.as_ptr() as usize, reference.len())
        .expect("cannot read timestamp");
        reference[0]
    }
}

pub fn open(path: String, flags: usize) -> usize {
    unsafe {
        let reference: &String = &path;
        syscall3(SYS_OPEN, reference.as_ptr() as usize, reference.len(), flags)
        .expect("cannot open the file")
    }
}
pub fn read(fd: usize, buf: &Vec<u8>) {
    unsafe {
        let reference: &Vec<u8> = &buf;
        syscall3(SYS_READ, fd, reference.as_ptr() as usize, reference.len())
        .expect("cannot open the file");
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        use crate::syscall::{call::*, number::*};
        unsafe {
            let content = format!("{}", format_args!($($arg)*));
            let reference: &str = content.as_str().as_ref();

            syscall2(SYS_DEBUG, content.as_str().as_ptr() as usize, reference.len())
            .expect("cannot debug");
        }
    };
}
