#![no_std]
#![feature(abi_x86_interrupt)]

pub mod util;
pub mod io;
pub mod interruptions;
pub mod gdt;

pub fn init() { 
    gdt::init();
    interruptions::init_idt();
}
