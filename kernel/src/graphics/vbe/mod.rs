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

use self::colors::*;
use self::geom::Shapes;
use crate::VBE_BUFFER;
use crate::memory::{map, paging::EntryFlags};

pub mod colors;
pub mod geom;

pub static FB_WIDTH: u32  = 1024;
pub static FB_HEIGHT: u32 = 768;
pub static FB_DEPTH: u32  = 4;

// TODO: To use text, see the file printer/vga_320x200.rs in the bootloader crate
pub fn init() {
    println!("Vbe driver is starting...");
    unsafe {
        let addr = *VBE_BUFFER.lock();
        map(addr as usize,
            addr as usize +
            FB_WIDTH as usize * FB_HEIGHT as usize * FB_DEPTH as usize,
            EntryFlags::PRESENT | EntryFlags::WRITABLE);
    }
    // Wallpaper
    Shapes::Rect {
        x: 0,
        y: 0,
        w: FB_WIDTH,
        h: FB_HEIGHT - 30,
        color: Rgb::new(78, 193, 255)
    }.draw();

    // Taskbar
    Shapes::Rect {
        x: 0,
        y: FB_HEIGHT - 30,
        w: FB_WIDTH,
        h: 30,
        color: Rgb::new(255, 110, 26)
    }.draw();

    // Menu
    Shapes::Rect {
        x: 0,
        y: FB_HEIGHT - 30,
        w: 30,
        h: 30,
        color: Rgb::new(0, 170, 255)
    }.draw();


    // Windows
    Shapes::Rect {
        x: 10,
        y: 10,
        w: 300,
        h: 200,
        color: Rgb::new(57, 57, 57)
    }.draw();
    Shapes::Rect {
        x: FB_WIDTH - 450,
        y: FB_HEIGHT - 350,
        w: 400,
        h: 300,
        color: Rgb::new(57, 57, 57)
    }.draw();
}

