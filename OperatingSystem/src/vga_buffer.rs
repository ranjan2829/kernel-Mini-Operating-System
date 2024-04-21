#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[repr(transparent)]
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
struct ColorCode(u8);

impl ColorCode {
    fn new(ForeGorund: Color, Background: Color) -> ColorCode {
        ColorCode((Background as u8) << 4 | (ForeGorund as u8))
    }
}
