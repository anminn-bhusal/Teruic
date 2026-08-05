// For now this is just in testing phase for shell
// as this is kernel lvl shell so it may be temporary or not as well so idk the future.

// Todo: many things i even dont know but at first creating commands for further development


use crate::{print, println};

const COMMAND_BUFFER_SIZE: usize = 256;

pub struct Shell {
    buffer: [u8; COMMAND_BUFFER_SIZE],
    index: usize,
}

impl Shell {
    pub const fn new() -> Self {
        Self {
            buffer: [0; COMMAND_BUFFER_SIZE],
            index: 0,
        }
    }

    /// Handles an incoming character typed by the user
    pub fn handle_key(&mut self, c: char) {
        match c {
            // Enter key: Execute command
            '\n' => {
                println!();
                self.execute_command();
                self.clear_buffer();
                print!("teruic> ");
            }
            // Backspace key: Erase previous character from buffer and screen
            '\x08' => {
                if self.index > 0 {
                    self.index -= 1;
                    self.buffer[self.index] = 0;
                    print!("\x08"); // Pass 0x08 to VGA driver to clear screen cell
                }
            }
            // Standard printable ASCII character
            character if character.is_ascii() => {
                if self.index < COMMAND_BUFFER_SIZE - 1 {
                    self.buffer[self.index] = character as u8;
                    self.index += 1;
                    print!("{}", character);
                }
            }
            _ => {} // Ignore non-ASCII key events
        }
    }

    fn clear_buffer(&mut self) {
        self.buffer = [0; COMMAND_BUFFER_SIZE];
        self.index = 0;
    }

    /// Command Parser
    fn execute_command(&mut self) {
        if self.index == 0 {
            return;
        }

        // Convert byte slice into valid ASCII string slice
        let raw_cmd = match core::str::from_utf8(&self.buffer[..self.index]) {
            Ok(s) => s.trim(),
            Err(_) => return,
        };

        if raw_cmd == "help" {
            println!("Teruic OS Available Commands:");
            println!("  help     - Show this menu");
            println!("  clear    - Clear the screen buffer");
            println!("  info     - Display kernel system info");
            println!("  echo     - Echo back text (e.g. echo hello)");
            println!("  panic    - Trigger a test kernel panic");
        } else if raw_cmd == "clear" {
            crate::vga::clear_screen();
        } else if raw_cmd == "info" {
            println!("Teruic Kernel v0.1.0 (x86_64 Bare-Metal)");
            println!("Architecture: x86_64 Long Mode");
            println!("Interrupts  : 8259 PIC + IDT Enabled");
        } else if raw_cmd.starts_with("echo ") {
            let message = &raw_cmd[5..];
            println!("{}", message);
        } else if raw_cmd == "echo" {
            println!();
        } else if raw_cmd == "panic" {
            panic!("Manual kernel panic triggered from shell command!");
        } else {
            println!("Unknown command: '{}'. Type 'help' for available commands.", raw_cmd);
        }
    }
}