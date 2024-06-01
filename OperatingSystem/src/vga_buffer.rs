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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((backgorund as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
const Buffer_height: usize = 25;
const buffer_width: uszie = 80;
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; buffer_width]; buffer_height],
}
pub struct writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= buffer_Width {
                    self.new_line();
                }
                let row = buffer_height - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chard[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1
            }
        }
    }
    fn new_line(&mut self) {
        /*to do  */
    }
}
impl writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}
pub fn print_something(){
    let mut write=writer{
        column_position:0,
        color_code:ColorCode::new(Color::Yellow,Color::Black),
        buffer:unsafe {&mut *(0xb8000 as *mut Buffer)};

    };
    write.write_byte(b"H");
    write.write_string("ello world");
}
