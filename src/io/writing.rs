#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BgColor {
    Black = 0b000,
    Blue = 0b001,
    Green = 0b010,
    Cyan = 0b011,
    Red = 0b100,
    Magenta = 0b101,
    Brown = 0b110,
    White = 0b111,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8); 

impl ColorCode {
    //No need to specify background as black is 0
    const NEUTRAL_COLOR_CODE: ColorCode = ColorCode(Color::White as u8);

    pub fn new(foreground: Color, background: BgColor, blink: bool) -> ColorCode {
        ColorCode((foreground as u8) | (background as u8) << 4 | if blink {1} else {0} << 7)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    const BLANK: ScreenChar = ScreenChar { ascii: ' ' as u8, color_code: ColorCode::NEUTRAL_COLOR_CODE };

    pub fn new(ascii: u8, color_code: ColorCode) -> ScreenChar {
        ScreenChar { ascii, color_code }
    }

    pub fn white_char(ascii: u8) -> ScreenChar {
        ScreenChar::new(ascii, ColorCode::NEUTRAL_COLOR_CODE)
    }
}

const DEFAULT_VGA_BUFFER_ADDRESS: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
struct Buffer {
    //Needs to be volatile to ensure writes
    chars: [Volatile<ScreenChar>; BUFFER_WIDTH * BUFFER_HEIGHT]
}

impl Buffer {
    fn to_raw_ptr(&mut self) -> *mut ScreenChar {
        self as *mut _ as *mut ScreenChar
    }
}

pub struct Writer {
    color_code: ColorCode,
    curr_position: usize,
    buffer: &'static mut Buffer,
}

use spin::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default_writer());
}

impl Writer {
    const DEFAULT_VGA_BUFFER_ADDRESS: usize = 0xb8000;
    const NOT_VALID_ASCII_CHAR: u8 = 0xfe;

    pub fn default_writer() -> Writer {
        Writer { 
            color_code: ColorCode::NEUTRAL_COLOR_CODE,
            curr_position: 0,
            buffer: unsafe { &mut *(DEFAULT_VGA_BUFFER_ADDRESS as *mut Buffer) }
        }
    }

    fn get_lines_written(&self) -> usize {
        self.curr_position / BUFFER_WIDTH
    }

    fn needs_shift_up(&self) -> bool {
        let next_line = self.get_lines_written() + 1;
        next_line > BUFFER_HEIGHT
    }

    fn clean_line(&mut self, line: usize) {
        assert!(line < BUFFER_HEIGHT);
        let starting_pos = (line * BUFFER_WIDTH) as usize;
        let ending_pos = ((line + 1) * BUFFER_WIDTH) as usize;

        for index in starting_pos..ending_pos {
            self.buffer.chars[index].write(ScreenChar::BLANK);
        }
    }

    fn shift_up(&mut self) {
        let pointer = self.buffer.to_raw_ptr();
        unsafe {
            crate::util::algorithm::copy(
                //Copy from second line till the end
                pointer.offset(BUFFER_WIDTH as isize),
                pointer.offset((BUFFER_WIDTH * BUFFER_HEIGHT) as isize),
                //To the first line (effectively, it removes first line)
                pointer
            );
        }
        //Clean last line, that still has contents of the last last line
        self.clean_line(BUFFER_HEIGHT - 1);

        //Set position to last line
        self.curr_position = BUFFER_WIDTH * (BUFFER_HEIGHT - 1)
    }

    fn shift_up_if_needed(&mut self) {
        if self.needs_shift_up() { self.shift_up() };
    }

    fn new_line(&mut self) {
        self.curr_position = (self.get_lines_written() + 1) * BUFFER_WIDTH;
        self.shift_up_if_needed();
    }

    pub fn write_byte_color(&mut self, byte: u8, color: ColorCode) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                self.buffer.chars[self.curr_position].write(
                    ScreenChar {
                        ascii: byte,
                        color_code: color,
                    }
                );
                self.curr_position += 1;
                self.shift_up_if_needed();
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.write_byte_color(byte, self.color_code);
    }
}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                //Print only valid characters, which are in the range below or
                //the \n char
                //else, print the specified char
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(Self::NOT_VALID_ASCII_CHAR),
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::writing::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(
        || WRITER.lock().write_fmt(args).unwrap()
    );
}
