// For now this is just in testing phase for shell
// as this is kernel lvl shell so it may be temporary or not as well so idk the future.

use crate::{print, println};

const COMMAND_BUFFER_SIZE: usize = 256;

pub struct Shell {
    buffer: [char; COMMAND_BUFFER_SIZE],
    index: usize,
}

impl Shell {
    pub const fn new() -> Self {
        Self {
            buffer: ['\0'; COMMAND_BUFFER_SIZE],
            index: 0,
        }
    }

    /// Handles a incoming character typed by the user
    pub fn handle_key(&mut self, c: char) {
        match c {
            // Enter key: Execute command
            '\n' => {
                println!();
                self.execute_command();
                self.clear_buffer();
                print!("teruic> ");
            }
            // Backspace key: Erase previous character
            '\x08' => {
                if self.index > 0 {
                    self.index -= 1;
                    self.buffer[self.index] = '\0';
                    // Send backspace + space + backspace to clear VGA screen character
                    print!("\x08 \x08");
                }
            }
            // Standard printable character
            character => {
                if self.index < COMMAND_BUFFER_SIZE - 1 {
                    self.buffer[self.index] = character;
                    self.index += 1;
                    print!("{}", character);
                }
            }
        }
    }

    fn clear_buffer(&mut self) {
        self.buffer = ['\0'; COMMAND_BUFFER_SIZE];
        self.index = 0;
    }

    /// Command Parser
    fn execute_command(&mut self) {
        if self.index == 0 {
            return;
        }

        // Convert key buffer into slice for inspection
        let cmd = &self.buffer[..self.index];

        if matches_str(cmd, "help") {
            println!("Teruic OS Available Commands:");
            println!("  help     - Show this menu");
            println!("  clear    - Clear the screen buffer");
            println!("  info     - Display kernel system info");
            println!("  panic    - Trigger a test kernel panic");
        } else if matches_str(cmd, "clear") {
            crate::vga::clear_screen();
        } else if matches_str(cmd, "info") {
            println!("Teruic Kernel v0.1.0 (x86_64 Bare-Metal)");
            println!("Architecture: x86_64 Long Mode");
            println!("Interrupts  : 8259 PIC + IDT Enabled");
        } else if matches_str(cmd, "panic") {
            panic!("Manual kernel panic triggered from shell command!");
        } else {
            print!("Unknown command: '");
            for &ch in cmd {
                print!("{}", ch);
            }
            println!("'. Type 'help' for available commands.");
        }
    }
}

/// Helper function to compare `char` buffer against string literal without heap allocations
fn matches_str(buf: &[char], expected: &str) -> bool {
    if buf.len() != expected.len() {
        return false;
    }
    for (i, e) in expected.chars().enumerate() {
        if buf[i] != e {
            return false;
        }
    }
    true
}