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

pub fn switch_to_usermode(){
       /*__asm__ ("movl %%esp,%%eax\n\t" \
			 "pushl %%ecx\n\t" \
			 "pushl %%eax\n\t" \
			 "pushfl\n\t" \
			 "pushl %%ebx\n\t" \
			 "pushl $1f\n\t" \
			 "iret\n" \
			 "1:\tmovw %%cx,%%ds\n\t" \
			 "movw %%cx,%%es\n\t" \
			 "movw %%cx,%%fs\n\t" \
			 "movw %%cx,%%gs" \
			 ::"b"(USER_CODE_SEL),"c"(USER_DATA_SEL));
*/

/*
asm volatile(
			"mov %3, %%esp\n"
			"pushl $0xDECADE21\n"  /* Magic */
			"mov $0x23, %%ax\n"    /* Segment selector */
			"mov %%ax, %%ds\n"
			"mov %%ax, %%es\n"
			"mov %%ax, %%fs\n"
			"mov %%ax, %%gs\n"
			"mov %%esp, %%eax\n"   /* Stack -> EAX */
			"pushl $0x23\n"        /* Segment selector again */
			"pushl %%eax\n"
			"pushf\n"              /* Push flags */
			"popl %%eax\n"         /* Fix the Interrupt flag */
			"orl  $0x200, %%eax\n"
			"pushl %%eax\n"
			"pushl $0x1B\n"
			"pushl %0\n"           /* Push the entry point */
			"iret\n"
			: : "m"(location), "r"(argc), "r"(argv), "r"(stack) : "%ax", "%esp", "%eax");
*/
}
pub fn init_user(){
	println!("Switch");
	switch_to_usermode();
	println!("Success! You are now in user mode!");
}
