#![no_std]
#![no_main]

mod util;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Welcome to RustOS\nHow may I help you\n";

use util::writing::{Writer, ColorCode, Color, BgColor};

const DEFAULT_VGA_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_WIDTH: usize = 80;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::default_writer();
    for (i, &byte) in HELLO.iter().enumerate() {
        writer.write_byte(byte);
    }

    let mut c = 'a';
    loop{
        for _ in 0..5000 {}
        writer.write_byte(c as u8);

        c = ((c as u8) + 1) as char;
        if c > 'z' { c = 'a' };
    }
}
