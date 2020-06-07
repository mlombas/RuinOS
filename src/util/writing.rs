#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0 << 4,
    Blue = 1 << 4,
    Green = 2 << 4,
    Cyan = 3 << 4,
    Red = 4 << 4,
    Magenta = 5 << 4,
    Brown = 6 << 4,
    LightGray = 7 << 4,
    DarkGray = 8 << 4,
    LightBlue = 9 << 4,
    LightGreen = 10 << 4,
    LightCyan = 11 << 4,
    LightRed = 12 << 4,
    Pink = 13 << 4,
    Yellow = 14 << 4,
    White = 15 << 4,
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
    pub fn new(foreground: Color, background: BgColor, blink: bool) -> ColorCode {
        ColorCode((foreground as u8) << 4 | (background as u8) << 1 | if blink { 1 } else { 0 });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(ascii: u8, color_code: ColorCode) -> ScreenChar {
        ScreenChar { ascii, color_code }
    }

    pub fn white_char(ascii: u8) -> ScreenChar {
        ScreenChar::new(ascii, ColorCode::new(Color::White, BgColor::Black, false))
    }
}
