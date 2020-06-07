#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static DEFAULT_VGA_BUFFER_ADDRESS: u64 = 0xb8000;

static BLUE:     u8 = 0b000001;
static GREEN:    u8 = 0b000010;
static RED:      u8 = 0b000100;
static BG_BLUE:  u8 = 0b001000;
static BG_GREEN: u8 = 0b010000;
static BG_RED:   u8 = 0b100000;

struct VgaWriter {
    curr_color: u8,
    vga_buffer: *mut u8, 
}

impl VgaWriter {
    fn new() -> VgaWriter {
        VgaWriter {
            curr_color: BLUE | GREEN | RED,
            vga_buffer: DEFAULT_VGA_BUFFER_ADDRESS as *mut u8,
        }
    }

    fn change_color(&mut self, new_color: u8) {
        self.curr_color = new_color;
    }

    fn write(&self, text: &[u8]) {
        for (i, &byte) in text.iter().enumerate() {
            unsafe {
                *self.vga_buffer.offset(i as isize * 2) = byte;
                *self.vga_buffer.offset(i as isize * 2 + 1) = self.curr_color;
            }
        }
    }
}

static HELLO: &[u8] = b"Welcome to RuinOS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let changes = [BLUE | GREEN | RED, BLUE | GREEN | BG_RED, GREEN | RED, BG_BLUE | BG_RED | BG_GREEN, RED | BLUE | BG_RED, GREEN | BG_RED | BG_GREEN];
    let mut index = 0;
    let mut writer = VgaWriter::new();

    loop{
        writer.change_color(changes[index]);
        index = (index + 1) % changes.len();

        writer.write(HELLO);

        for _ in 0..50000 {}
    }
}
