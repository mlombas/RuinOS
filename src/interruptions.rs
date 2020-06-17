pub mod pic;

use lazy_static::lazy_static;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use crate::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

//Exceptions

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame
) {
    println!("Breakpoint:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64
) -> ! {
    panic!("Double fault:\n{:#?}", stack_frame);
}

//Interrupts

use pic::Pics;
use spin;

const PIC_1_OFFSET: usize = 32;
const PIC_2_OFFSET: usize = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<Pics> = 
    spin::Mutex::new( unsafe { Pics::new(PIC_1_OFFSET, PIC_2_OFFSET) } );

pub fn init_interrupts() {
    unsafe { PICS.lock().init(); }
    x86_64::instructions::interrupts::enable();
}
