// For now this is just in testing phase for shell
// as this is kernel lvl shell so it may be temporary or not as well so idk the future.

// Todo: many things i even dont know but at first creating commands for further development

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::{print, println};

pub struct Shell {
    // Both of these use the Heap! 
    // They grow dynamically as you type or run more commands.
    buffer: String,
    history: Vec<String>,
}

impl Shell {
    pub const fn new() -> Self {
        Self {
            buffer: String::new(),
            history: Vec::new(),
        }
    }

    pub fn handle_key(&mut self, c: char) {
        match c {
            '\n' => {
                println!();
                self.execute_command();
                self.buffer.clear();
                print!("teruic> ");
            }
            '\x08' => {
                // pop() removes the last character from the String dynamically
                if self.buffer.pop().is_some() {
                    print!("\x08"); 
                }
            }
            character if character.is_ascii() => {
                // push() allocates more memory on the heap if the string gets too long!
                self.buffer.push(character);
                print!("{}", character);
            }
            _ => {} 
        }
    }

    fn execute_command(&mut self) {
        if self.buffer.trim().is_empty() {
            return;
        }

        // 1. Save to command history (Allocates a new String on the heap)
        self.history.push(self.buffer.clone());

        // 2. Split the buffer into arguments using Vec (Allocates a list on the heap)
        let args: Vec<&str> = self.buffer.trim().split_whitespace().collect();
        let cmd = args[0];

        match cmd {
            "help" => {
                println!("Teruic OS Available Commands:");
                println!("  help     - Show this menu");
                println!("  clear    - Clear the screen buffer");
                println!("  info     - Display kernel system info");
                println!("  history  - Show command history");
                println!("  echo     - Echo back text (e.g. echo hello)");
            }
            "clear" => crate::vga::clear_screen(),
            "info" => {
                println!("Teruic Kernel v0.1.0 (x86_64 Bare-Metal)");
                println!("Heap Allocator: ACTIVE and functioning");
            }
            "history" => {
                println!("Command History:");
                for (i, past_cmd) in self.history.iter().enumerate() {
                    println!("  {}: {}", i + 1, past_cmd);
                }
            }
            "echo" => {
                if args.len() > 1 {
                    // Joins arguments with a space (Allocates a new String on the heap)
                    let msg = args[1..].join(" ");
                    println!("{}", msg);
                }
            }
            _ => println!("Unknown command: '{}'. Type 'help' for available commands.", cmd),
        }
    }
}