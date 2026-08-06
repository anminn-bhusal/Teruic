//graphical Text Buffer & UI Rendering Base Layer

use alloc::format;
use crate::vga::{Color, WRITER};

pub struct UI;

impl UI {
    /// Renders a modern styled desktop banner at the top of the screen
    pub fn draw_header(title: &str) {
        let mut writer = WRITER.lock();

        // Render top status bar in Dark Gray with White Text
        writer.set_colors(Color::White, Color::DarkGray);
        writer.clear_row(0);

        let header = format!("  [Teruic OS v0.1.0] | Application Layer: Active | System: {}", title);
        writer.write_string_at(0, 0, &header);

        // Reset colors back for terminal output
        writer.set_colors(Color::LightGreen, Color::Black);
    }

    /// Renders a decorated UI card/box for running applications
    pub fn draw_window(title: &str, content: &[&str]) {
        crate::println!("\n+---------------------------------------------------+");
        crate::println!("| APP: {:<45} |", title);
        crate::println!("+---------------------------------------------------+");
        for line in content {
            crate::println!("| {:<49} |", line);
        }
        crate::println!("+---------------------------------------------------+\n");
    }
}