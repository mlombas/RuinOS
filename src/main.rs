#![no_std]
#![no_main]

mod util;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static DEFAULT_VGA_BUFFER_ADDRESS: u64 = 0xb8000;

static HELLO: &[u8] = b"Welcome to RuinOS";

use util::writing::ScreenChar;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = DEFAULT_VGA_BUFFER_ADDRESS as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            vga_buffer.offset(i as isize * 2) = ScreenChar::white_char(byte) as u16;
        }
    }

    loop{}
}
