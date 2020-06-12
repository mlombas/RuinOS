#![no_std]
#![no_main]

mod util;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &'static str = "Welcome to RustOS\nHow may I help you\n";

use util::writing::{Writer, ColorCode, Color, BgColor};

const DEFAULT_VGA_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_WIDTH: usize = 80;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    let mut writer = Writer::default_writer();
    writer.write_str(HELLO);
    writer.write_str("henlö");
    write!(writer, "Does this wörk?");

    loop{ }
}
