// text editor making

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::vfs::VFS;
use crate::vga::{Color, WRITER};

pub struct TextEditor {
    pub active: bool,
    pub filename: String,
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    clipboard: String,
}

impl TextEditor {
    pub const fn new() -> Self {
        Self {
            active: false,
            filename: String::new(),
            lines: Vec::new(),
            cursor_x: 0,
            cursor_y: 0,
            clipboard: String::new(),
        }
    }

    pub fn open(&mut self, filename: &str) {
        self.filename = String::from(filename);
        self.active = true;
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.clipboard.clear();
        self.lines.clear();

        if let Some(bytes) = VFS.lock().read_file(filename) {
            if let Ok(content) = core::str::from_utf8(&bytes) {
                for line in content.lines() {
                    self.lines.push(String::from(line));
                }
            }
        }

        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        self.render_full_screen();
    }

    pub fn handle_key(&mut self, c: char) {
        if !self.active {
            return;
        }

        if self.cursor_y >= self.lines.len() {
            self.cursor_y = self.lines.len().saturating_sub(1);
        }
        if self.cursor_x > self.lines[self.cursor_y].len() {
            self.cursor_x = self.lines[self.cursor_y].len();
        }

        match c {
            // Save & Exit Shortcut (Ctrl+S '\x13', Esc '\x1b', or Backtick '`')
            '\x13' | '\x1b' | '`' => {
                self.save();
                self.close();
                return;
            }
            // Ctrl+A: Jump to start of document
            '\x01' => {
                self.cursor_x = 0;
                self.cursor_y = 0;
            }
            // Ctrl+C: Copy current line to clipboard
            '\x03' => {
                if self.cursor_y < self.lines.len() {
                    self.clipboard = self.lines[self.cursor_y].clone();
                }
            }
            // Ctrl+V: Paste clipboard content at cursor
            '\x16' => {
                if !self.clipboard.is_empty() {
                    let clip = self.clipboard.clone();
                    self.lines[self.cursor_y].insert_str(self.cursor_x, &clip);
                    self.cursor_x += clip.len();
                }
            }
            // Left Arrow
            '\u{E000}' | '\x11' => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.lines[self.cursor_y].len();
                }
            }
            // Right Arrow
            '\u{E001}' | '\x12' => {
                if self.cursor_x < self.lines[self.cursor_y].len() {
                    self.cursor_x += 1;
                } else if self.cursor_y + 1 < self.lines.len() {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            }
            // Up Arrow
            '\u{E002}' | '\x14' => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    if self.cursor_x > self.lines[self.cursor_y].len() {
                        self.cursor_x = self.lines[self.cursor_y].len();
                    }
                }
            }
            // Down Arrow
            '\u{E003}' | '\x15' => {
                if self.cursor_y + 1 < self.lines.len() {
                    self.cursor_y += 1;
                    if self.cursor_x > self.lines[self.cursor_y].len() {
                        self.cursor_x = self.lines[self.cursor_y].len();
                    }
                }
            }
            '\n' | '\r' => {
                let current_line = &self.lines[self.cursor_y];
                let remainder = current_line[self.cursor_x..].to_string();
                self.lines[self.cursor_y].truncate(self.cursor_x);

                self.cursor_y += 1;
                self.lines.insert(self.cursor_y, remainder);
                self.cursor_x = 0;
            }
            '\x08' | '\x7f' => {
                // Backspace
                if self.cursor_x > 0 {
                    self.lines[self.cursor_y].remove(self.cursor_x - 1);
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    let prev_line_len = self.lines[self.cursor_y - 1].len();
                    let current_line = self.lines.remove(self.cursor_y);
                    self.cursor_y -= 1;
                    self.lines[self.cursor_y].push_str(&current_line);
                    self.cursor_x = prev_line_len;
                }
            }
            character if character >= ' ' && character <= '~' => {
                self.lines[self.cursor_y].insert(self.cursor_x, character);
                self.cursor_x += 1;
            }
            _ => {}
        }

        self.render_full_screen();
    }

    pub fn save(&self) {
        let mut full_text = String::new();
        for (i, line) in self.lines.iter().enumerate() {
            full_text.push_str(line);
            if i + 1 < self.lines.len() {
                full_text.push('\n');
            }
        }
        VFS.lock().write_file(&self.filename, full_text.into_bytes());
    }

    pub fn delete_file(&mut self) {
        let filename_clone = self.filename.clone();
        VFS.lock().remove_file(&filename_clone);

        self.lines.clear();
        self.lines.push(String::new());
        self.active = false;
        
        crate::vga::clear_screen();
        crate::gui::UI::draw_header("Terminal Shell");
        crate::println!("\n[Permanently deleted file '{}']", filename_clone);
        crate::print!("teruic> ");
    }

    pub fn close(&mut self) {
        self.active = false;
        crate::vga::clear_screen();
        crate::gui::UI::draw_header("Terminal Shell");
        crate::println!("\n[Saved file '{}' to VFS]", self.filename);
        crate::print!("teruic> ");
    }

    pub fn render_full_screen(&self) {
        let mut writer = WRITER.lock();

        // 1. Status Header Bar
        writer.set_colors(Color::Black, Color::LightGray);
        writer.clear_row(0);
        let header = format!(" TeruicPad v1.0 | File: {:<20} | [`] Save & Exit ", self.filename);
        let padded_header = if header.len() < 80 {
            format!("{:<80}", header)
        } else {
            header[..80].to_string()
        };
        writer.write_string_at(0, 0, &padded_header);

        // 2. Clear canvas rows
        writer.set_colors(Color::White, Color::Black);
        for row in 1..24 {
            writer.clear_row(row);
        }

        // 3. Scroll computation
        let visible_height = 22;
        let scroll_top = if self.cursor_y >= visible_height {
            self.cursor_y - visible_height + 1
        } else {
            0
        };

        // 4. Render lines
        for screen_row in 1..24 {
            let line_idx = scroll_top + (screen_row - 1);
            if line_idx < self.lines.len() {
                let line = &self.lines[line_idx];
                let display_len = core::cmp::min(line.len(), 80);
                
                writer.set_colors(Color::LightGray, Color::Black);
                writer.write_string_at(screen_row, 0, &line[..display_len]);

                // Render block cursor if active row
                if line_idx == self.cursor_y && self.cursor_x < 80 {
                    let cursor_char = if self.cursor_x < line.len() {
                        line.chars().nth(self.cursor_x).unwrap_or(' ')
                    } else {
                        ' '
                    };
                    
                    writer.set_colors(Color::Black, Color::White);
                    let cursor_str = format!("{}", cursor_char);
                    writer.write_string_at(screen_row, self.cursor_x, &cursor_str);
                }
            }
        }

        // 5. Footer Bar
        writer.set_colors(Color::Black, Color::LightCyan);
        writer.clear_row(24);
        let footer = format!(
            " Ctl+C:Copy | Ctl+V:Paste | Ctl+A:Start | Ln {}, Col {} ",
            self.cursor_y + 1,
            self.cursor_x + 1
        );
        let padded_footer = if footer.len() < 80 {
            format!("{:<80}", footer)
        } else {
            footer[..80].to_string()
        };
        writer.write_string_at(24, 0, &padded_footer);
        writer.set_colors(Color::LightGreen, Color::Black);
    }
}

pub static mut EDITOR: TextEditor = TextEditor::new();