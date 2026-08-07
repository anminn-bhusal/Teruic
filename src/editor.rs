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

    /// Open or create a file in full-screen edit mode
    pub fn open(&mut self, filename: &str) {
        self.filename = String::from(filename);
        self.active = true;
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.lines.clear();

        // Load existing file or start with an empty line
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

    /// Handle keypresses while in editor mode
    pub fn handle_key(&mut self, c: char) {
        if !self.active {
            return;
        }

        match c {
            // Save & Exit Shortcut (Backtick ` or Ctrl+S or Esc)
            '\x13' | '\x1b' | '`' => {
                self.save();
                self.close();
                return;
            }
            '\n' => {
                let current_line = &self.lines[self.cursor_y];
                let remainder = current_line[self.cursor_x..].to_string();
                self.lines[self.cursor_y].truncate(self.cursor_x);

                self.cursor_y += 1;
                self.lines.insert(self.cursor_y, remainder);
                self.cursor_x = 0;
            }
            '\x08' => {
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
            character if character.is_ascii() && !character.is_control() => {
                self.lines[self.cursor_y].insert(self.cursor_x, character);
                self.cursor_x += 1;
            }
            _ => {}
        }

        self.render_full_screen();
    }

    /// Save the editor contents back into the Virtual File System
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

    /// Exit editor mode and restore the shell layout
    pub fn close(&mut self) {
        self.active = false;
        crate::vga::clear_screen();
        crate::gui::UI::draw_header("Terminal Shell");
        crate::println!("\n[Saved file '{}' to VFS]", self.filename);
        crate::print!("teruic> ");
    }

    /// Redraw the editor interface onto VGA buffer
    pub fn render_full_screen(&self) {
        let mut writer = WRITER.lock();

        // Top Status Header
        writer.set_colors(Color::Black, Color::LightGray);
        writer.clear_row(0);
        let header = format!("  Teruic EDIT v1.0 | Editing: {} ", self.filename);
        writer.write_string_at(0, 0, &header);

        // Clear canvas rows 1..23
        writer.set_colors(Color::LightGreen, Color::Black);
        for row in 1..24 {
            writer.clear_row(row);
        }

        // Render editor text lines
        for (idx, line) in self.lines.iter().enumerate() {
            if idx >= 23 {
                break;
            }
            writer.write_string_at(idx + 1, 0, line);
        }

        // Bottom Control Bar
        writer.set_colors(Color::White, Color::DarkGray);
        writer.clear_row(24);
        let footer = format!(
            " [`] Save & Exit | Line {}, Col {} ",
            self.cursor_y + 1,
            self.cursor_x + 1
        );
        writer.write_string_at(24, 0, &footer);

        // Reset colors
        writer.set_colors(Color::LightGreen, Color::Black);
    }
}

pub static mut EDITOR: TextEditor = TextEditor::new();