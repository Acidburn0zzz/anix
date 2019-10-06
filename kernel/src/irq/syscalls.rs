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

use x86::msr;
use gdt;
use syscall::number::*;
use errors::*;

pub unsafe fn init() {
    msr::wrmsr(msr::IA32_STAR, ((gdt::GDT_KERNEL_CODE as u64) << 3) << 32);
    msr::wrmsr(msr::IA32_LSTAR, syscall_instruction as u64);
    msr::wrmsr(msr::IA32_FMASK, 1 << 9);
    msr::wrmsr(msr::IA32_KERNEL_GSBASE, &gdt::TSS as *const _ as u64);

    let efer = msr::rdmsr(msr::IA32_EFER);
    msr::wrmsr(msr::IA32_EFER, efer | 1);
}

pub unsafe extern fn do_syscall(num: usize, arg1: usize, arg2: usize, _arg3: usize, _arg4: usize,
                                _arg5: usize, _bp: usize, _stack: &mut SyscallStack)
                                -> usize {
    match num {
        SYS_EXIT => {
            // TODO: Kill or stop the task
            use task::{scheduler::*, *};
            kill();
            switch();
            0
        },
        SYS_TIME => {
            use device::rtc::Rtc;
            use core::slice::from_raw_parts_mut;
            use time::DateTime;

            let pointer = from_raw_parts_mut(arg1 as *mut DateTime, arg2);
            pointer[0] = Rtc::new().date();
            0
        },
        SYS_DEBUG => {
            use core::slice::from_raw_parts;
            use core::str::from_utf8;

            println!("{}", from_utf8(from_raw_parts(arg1 as *const u8, arg2)).expect("cannot transform to utf8"));
            0
        }
        _ => return ENOSYS as usize,
    }
}

#[naked]
pub unsafe extern fn syscall_instruction() {
    #[inline(never)]
    unsafe fn inner(stack: &mut SyscallStack) -> usize {
        let rbp;
        asm!("" : "={rbp}"(rbp) : : : "intel", "volatile");

        do_syscall(stack.rax, stack.rdi, stack.rsi, stack.rdx, stack.r10, stack.r8, rbp, stack)
    }

    // Yes, this is magic. No, you don't need to understand
    asm!("xchg bx, bx
          swapgs                    // Set gs segment to TSS
          mov gs:[28], rsp          // Save userspace rsp
          mov rsp, gs:[4]           // Load kernel rsp
          push 5 * 8 + 3            // Push userspace data segment
          push qword ptr gs:[28]    // Push userspace rsp
          mov qword ptr gs:[28], 0  // Clear userspace rsp
          push r11                  // Push rflags
          push 4 * 8 + 3            // Push userspace code segment
          push rcx                  // Push userspace return pointer
          swapgs                    // Restore gs
          "
          :
          :
          :
          : "intel", "volatile");

      // Push scratch registers
      asm!("push rax
           push rbx
           push rcx
           push rdx
           push rdi
           push rsi
           push r8
           push r9
           push r10
           push r11
           push fs
           mov r11, 0x18
           mov fs, r11"
           : : : : "intel", "volatile");

      // Get reference to stack variables
      let rsp: usize;
      asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

      let a = inner(&mut *(rsp as *mut SyscallStack));

      asm!("" : : "{rax}"(a) : : "intel", "volatile");

      // Interrupt return
      asm!("pop fs
            pop r11
            pop r10
            pop r9
            pop r8
            pop rsi
            pop rdi
            pop rdx
            pop rcx
            pop rbx
            add rsp, 8
            iretq"
            : : : : "intel", "volatile");
}

#[naked]
pub unsafe extern fn syscall() {
    #[inline(never)]
    unsafe fn inner(stack: &mut SyscallStack) -> usize {
        let rbp;
        asm!("" : "={rbp}"(rbp) : : : "intel", "volatile");

        do_syscall(stack.rax, stack.rbx, stack.rcx, stack.rdx,
                stack.rsi, stack.rdi, rbp, stack)
    }

    // Push scratch registers
    asm!("push rax
         push rbx
         push rcx
         push rdx
         push rdi
         push rsi
         push r8
         push r9
         push r10
         push r11
         push fs
         mov r11, 0x18
         mov fs, r11"
         : : : : "intel", "volatile");

    // Get reference to stack variables
    let rsp: usize;
    asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

    let a = inner(&mut *(rsp as *mut SyscallStack));

    asm!("" : : "{rax}"(a) : : "intel", "volatile");

    // Interrupt return
    asm!("pop fs
          pop r11
          pop r10
          pop r9
          pop r8
          pop rsi
          pop rdi
          pop rdx
          pop rcx
          pop rbx
          add rsp, 8
          iretq"
          : : : : "intel", "volatile");
}

#[allow(dead_code)]
#[repr(packed)]
pub struct SyscallStack {
    pub fs: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
}

