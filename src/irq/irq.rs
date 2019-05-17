use idt::*;
use super::irqid::*;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;
use user::input::{cmd_character, cmd_number};
use super::syscalls::*;

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
	use crate::screen::{WRITER, BUFFER_HEIGHT, BUFFER_WIDTH, ColorCode, Color};
	use crate::time::*;
	use crate::scheduler::schedule;
	
	//Increment time
	unsafe{
		time.deciseconds += 1;
	
		if time.deciseconds >= 19{
			time.seconds += 1;
			time.deciseconds = 0;
		}
	
		if time.seconds >= 59{
			time.minutes += 1;
			time.seconds = 0;
			time.deciseconds = 0;
			
		}
	
		if time.minutes >= 59{
			time.hours += 1;
			time.minutes = 0;
			time.seconds = 0;
			time.deciseconds = 0;
		}
		
		//Call the schedule function for switching task
		schedule();
		PICS.lock().notify_end_of_interrupt(TIMER_ID)
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use crate::screen::{WRITER, BUFFER_HEIGHT, BUFFER_WIDTH, ColorCode, Color};

    use pc_keyboard::*;
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
					unsafe{cmd_character(character)};
                },
                DecodedKey::RawKey(key) => {
					unsafe{cmd_number(key)};
				},
			}
		}
	}
	unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_ID) }
} 
pub extern "x86-interrupt" fn cascade_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\n\nCascade\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(CASCADE_ID) }
}

pub extern "x86-interrupt" fn com1_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nCom1\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(COM1_ID) }
}

pub extern "x86-interrupt" fn com2_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nCom2\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(COM2_ID) }
}

pub extern "x86-interrupt" fn lpt1_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nLpt1\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(LPT1_ID) }
}

pub extern "x86-interrupt" fn floppy_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nFloppy\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(FLOPPY_ID) }
}

pub extern "x86-interrupt" fn lpt2_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nLpt2\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(LPT2_ID) }
}

pub extern "x86-interrupt" fn rtc_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nRTC\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(RTC_ID) }
}

pub extern "x86-interrupt" fn pci1_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nPIC1\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(PCI1_ID) }
}

pub extern "x86-interrupt" fn pci2_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nPIC2\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(PCI2_ID) }
}

pub extern "x86-interrupt" fn pci3_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nPIC3\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(PCI3_ID) }
}

pub extern "x86-interrupt" fn mouse_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nMOUSE\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(MOUSE_ID) }
}

pub extern "x86-interrupt" fn fpu_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nFPU\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(FPU_ID) }
}

pub extern "x86-interrupt" fn ata1_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nATA1\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(ATA1_ID) }
}

pub extern "x86-interrupt" fn ata2_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nATA2\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(ATA2_ID) }
}

pub extern "x86-interrupt" fn disk_primary_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nDISK PRIMARY\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(DISK_PRIMARY_ID) }
}

pub extern "x86-interrupt" fn disk_secondary_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nDISK SECONDARY\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(DISK_SECONDARY_ID) }
}

pub extern "x86-interrupt" fn syscall_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\n\nSYSCALL\n{:#?}", stack_frame);
    unsafe{do_syscall();}
    print!("\nEND OF SYSCALL");
    unsafe { PICS.lock().notify_end_of_interrupt(SYSCALL_ID) }
    //TODO: Manage syscalls
}