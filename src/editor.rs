//text editor making
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::Mutex;
use crate::vfs::VFS;

pub static EDITOR: Mutex<TextEditor> = Mutex::new(TextEditor::new());

pub struct TextEditor {
    pub active: bool,
    pub filename: String,
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl TextEditor {
    pub const fn new() -> Self {
        Self {
            active: false,
            filename: String::new(),
            lines: Vec::new(),
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn open(&mut self, filename: &str) {
        self.filename = filename.to_string();
        self.active = true;
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.lines.clear();

        if let Some(bytes) = VFS.lock().read_file(filename) {
            if let Ok(text) = core::str::from_utf8(&bytes) {
                for line in text.lines() {
                    self.lines.push(line.to_string());
                }
            }
        }

        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        self.render();
    }

    pub fn save(&mut self) {
        let mut content = String::new();
        for (i, line) in self.lines.iter().enumerate() {
            content.push_str(line);
            if i + 1 < self.lines.len() {
                content.push('\n');
            }
        }
        VFS.lock().write_file(&self.filename, content.into_bytes());
    }

    pub fn close(&mut self) {
        self.active = false;
        self.lines.clear();
        self.filename.clear();
        crate::vga::clear_screen();
        crate::print!("teruic> ");
    }

    pub fn handle_key(&mut self, c: char) {
        if !self.active {
            return;
        }

        match c {
            // Ctrl+S or ASCII 19 -> Save
            '\x13' => {
                self.save();
                self.render();
            }
            // Ctrl+X or ASCII 24 / Esc -> Exit
            '\x18' | '\x1b' => {
                self.save();
                self.close();
            }
            // Arrow Left
            '\x11' => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.lines[self.cursor_y].len();
                }
                self.render();
            }
            // Arrow Right
            '\x12' => {
                if self.cursor_x < self.lines[self.cursor_y].len() {
                    self.cursor_x += 1;
                } else if self.cursor_y + 1 < self.lines.len() {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
                self.render();
            }
            // Arrow Up
            '\x14' => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    if self.cursor_x > self.lines[self.cursor_y].len() {
                        self.cursor_x = self.lines[self.cursor_y].len();
                    }
                }
                self.render();
            }
            // Arrow Down
            '\x15' => {
                if self.cursor_y + 1 < self.lines.len() {
                    self.cursor_y += 1;
                    if self.cursor_x > self.lines[self.cursor_y].len() {
                        self.cursor_x = self.lines[self.cursor_y].len();
                    }
                }
                self.render();
            }
            // Enter
            '\n' | '\r' => {
                let current_line = &mut self.lines[self.cursor_y];
                let remainder = current_line.split_off(self.cursor_x);
                self.cursor_y += 1;
                self.lines.insert(self.cursor_y, remainder);
                self.cursor_x = 0;
                self.render();
            }
            // Backspace
            '\x08' | '\x7f' => {
                if self.cursor_x > 0 {
                    self.lines[self.cursor_y].remove(self.cursor_x - 1);
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    let current = self.lines.remove(self.cursor_y);
                    self.cursor_y -= 1;
                    self.cursor_x = self.lines[self.cursor_y].len();
                    self.lines[self.cursor_y].push_str(&current);
                }
                self.render();
            }
            // Standard typing character
            character if character >= ' ' && character <= '~' => {
                self.lines[self.cursor_y].insert(self.cursor_x, character);
                self.cursor_x += 1;
                self.render();
            }
            _ => {}
        }
    }

    pub fn render(&self) {
        crate::vga::clear_screen();
        crate::println!("TeruicPad Editor - [{}] | Ctrl+S: Save, Esc/Ctrl+X: Exit", self.filename);
        crate::println!("--------------------------------------------------------------------------------");

        for (idx, line) in self.lines.iter().enumerate() {
            if idx == self.cursor_y {
                let mut rendered_line = line.clone();
                if self.cursor_x <= rendered_line.len() {
                    rendered_line.insert(self.cursor_x, '|');
                }
                crate::println!("{}", rendered_line);
            } else {
                crate::println!("{}", line);
            }
        }
    }
}