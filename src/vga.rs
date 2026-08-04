// This is for testing one so we don't mind it for using AI code or whatever
// for this as this is temporary just to test display functions.

// VGA is Visual graphics array and it treats the display as a
//  2D grid of 80 columns by 25 rows means 2000character cells

// every character cells takes up 2 bytes of memory just for info. hahaa



// The Starts from heree:::::
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

/// Standard 16-color palette supported by VGA text hardware.
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

/// Combines a foreground and background color into a single byte attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Represents a single character cell on the screen (ASCII char + Color attribute).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// VGA Text Buffer dimensions.
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// Represents the physical memory structure at memory address 0xB8000.
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// The main Writer struct managing cursor positions and writing to memory.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte (ASCII character) to the screen.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // Direct volatile write to hardware memory at 0xB8000
                unsafe {
                    core::ptr::write_volatile(
                        &mut self.buffer.chars[row][col],
                        ScreenChar {
                            ascii_character: byte,
                            color_code,
                        },
                    );
                }
                self.column_position += 1;
            }
        }
    }

    /// Writes a full string to the screen. Unsupported characters default to `■` (0xfe).
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII byte range or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Non-printable ASCII bytes mapped to block character
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Scrolls all lines up by 1 and clears the bottom row.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe {
                    let character = core::ptr::read_volatile(&self.buffer.chars[row][col]);
                    core::ptr::write_volatile(&mut self.buffer.chars[row - 1][col], character);
                }
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row by filling it with blank spaces.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            unsafe {
                core::ptr::write_volatile(&mut self.buffer.chars[row][col], blank);
            }
        }
    }
}

/// Implementation of Rust's standard formatting trait (`core::fmt::Write`).
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Global thread-safe static writer pointing directly to 0xB8000
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}