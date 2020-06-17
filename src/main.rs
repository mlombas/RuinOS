#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &'static str = "Welcome to RustOS\nHow may I help you\n";

use ruin_os::println;

const DEFAULT_VGA_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_WIDTH: usize = 80;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Henlo");

    ruin_os::init();

    loop{ }
}
