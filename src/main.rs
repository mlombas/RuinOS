#![no_std]
#![no_main]
#![allow(dead_code)]

use ruin_os::{print,println};
use ruin_os::util::halt_loop;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    halt_loop()
}

static HELLO: &'static str = "Welcome to RustOS\nHow may I help you\n";

const DEFAULT_VGA_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_WIDTH: usize = 80;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Henlo");

    ruin_os::init();

    let jaja = 0xdeadbeaf as *mut u32;
    unsafe { *jaja = 69 };

    halt_loop()
}
