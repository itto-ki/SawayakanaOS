const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HIGHT: usize = 25;

pub static mut VGA_BUFFER: VGABuffer = VGABuffer {
    buffer: [[
        VGACharacter{
            character: b' ',
            color: (ColorCode::Black as u8) << 4 | ColorCode::Black as u8};
        VGA_BUFFER_WIDTH]; VGA_BUFFER_HIGHT],
    x_pos: 0,
    y_pos: 0,
};

#[allow(dead_code)]
pub enum ColorCode {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    LightMangenta = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[derive(Clone, Copy)]
struct VGACharacter {
    character: u8,
    color: u8,
}

impl VGACharacter {
    // VGA Character Attribute
    // |   7   |   6   |   5   |   4   |  3  |  2  |  1  |  0  |
    // | blink |    Background color   |    Foreground color   |
    fn new(character: u8, foreground: ColorCode, background: ColorCode) -> Self {
        VGACharacter {
            character: character,
            color: (background as u8) << 4 | foreground as u8
        }
    }
}

pub struct VGABuffer {
    buffer: [[VGACharacter; VGA_BUFFER_WIDTH]; VGA_BUFFER_HIGHT],
    x_pos: usize,
    y_pos: usize,
}


impl VGABuffer {

    pub fn new_line(&mut self) -> () {
        if self.y_pos < VGA_BUFFER_HIGHT - 1 {
            self.x_pos = 0;
            self.y_pos += 1;
        } else {
            for y in 1..VGA_BUFFER_HIGHT {
                for x in 0..VGA_BUFFER_WIDTH {
                    self.buffer[y-1][x] = self.buffer[y][x];
                    if y == VGA_BUFFER_HIGHT - 1 {
                        self.buffer[y][x] = VGACharacter::new(b' ', ColorCode::Black, ColorCode::Black);
                    }
                }
            }
            self.x_pos = 0;
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> () {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                self.buffer[self.y_pos][self.x_pos] =
                    VGACharacter::new(byte, ColorCode::Cyan, ColorCode::Black);
                self.x_pos += 1;
                if self.x_pos >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }
            }
        }
    }

    pub fn write_str(&mut self, s: &str) -> () {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn flush(&mut self) -> () {
        let vga_text_buffer = 0xb8000 as *mut u8;
        for y in 0..VGA_BUFFER_HIGHT {
            for x in 0..VGA_BUFFER_WIDTH {
                unsafe {
                    *vga_text_buffer.offset((x + y * VGA_BUFFER_WIDTH) as isize * 2) =
                        self.buffer[y][x].character;
                    *vga_text_buffer.offset((x + y * VGA_BUFFER_WIDTH) as isize * 2 + 1) =
                        self.buffer[y][x].color;
                }
            }
        }
    }

    pub fn clear(&mut self) -> () {
        for y in 0..VGA_BUFFER_HIGHT {
            for x in 0..VGA_BUFFER_WIDTH {
                self.buffer[y][x] = VGACharacter::new(b' ', ColorCode::Black, ColorCode::Black);
            }
        }
        self.x_pos = 0;
        self.y_pos = 0;
    }
}
