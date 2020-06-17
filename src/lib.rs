#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(const_fn)]
#![allow(dead_code)]

pub mod util;
pub mod io;
pub mod interruptions;
pub mod gdt;

pub fn init() { 
    gdt::init();
    interruptions::init_idt();
    interruptions::init_interrupts();
}
