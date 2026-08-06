// For now this is just in testing phase for shell
// as this is kernel lvl shell so it may be temporary or not as well so idk the future.

// Todo: many things i even dont know but at first creating commands for further development
use alloc::string::String;
use alloc::vec::Vec;
use crate::{print, println};
use crate::vfs::VFS;

pub struct Shell {
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
                if self.buffer.pop().is_some() {
                    print!("\x08"); 
                }
            }
            character if character.is_ascii() => {
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

        self.history.push(self.buffer.clone());

        let args: Vec<&str> = self.buffer.trim().split_whitespace().collect();
        let cmd = args[0];

        match cmd {
            "help" => {
                println!("Teruic OS Available Commands:");
                println!("  help     - Show this menu");
                println!("  ls       - List files in Virtual File System");
                println!("  cat      - Read a file (e.g. cat hello.txt)");
                println!("  write    - Create a text file (e.g. write test.txt hello)");
                println!("  clear    - Clear screen");
                println!("  info     - System info");
                println!("  uptime   - System uptime");
                println!("  history  - Command history");
            }
            "ls" => {
                let files = VFS.lock().list();
                println!("VFS Directory Listing:");
                for file in files {
                    println!("  {}", file);
                }
            }
            "cat" => {
                if args.len() > 1 {
                    let filename = args[1];
                    match VFS.lock().read_file(filename) {
                        Some(bytes) => {
                            if let Ok(text) = core::str::from_utf8(&bytes) {
                                println!("{}", text);
                            } else {
                                println!("[Binary Data - {} bytes]", bytes.len());
                            }
                        }
                        None => println!("File not found: '{}'", filename),
                    }
                } else {
                    println!("Usage: cat <filename>");
                }
            }
            "write" => {
                if args.len() > 2 {
                    let filename = args[1];
                    let content = args[2..].join(" ");
                    VFS.lock().write_file(filename, content.as_bytes().to_vec());
                    println!("Wrote to file '{}'", filename);
                } else {
                    println!("Usage: write <filename> <content>");
                }
            }
            "clear" => crate::vga::clear_screen(),
            "info" => {
                println!("Teruic Kernel v0.1.0 (x86_64 Bare-Metal)");
                println!("Heap Allocator: ACTIVE");
                println!("Virtual File System: ACTIVE");
            }
            "uptime" => {
                let secs = crate::interrupts::uptime_seconds();
                let ticks = crate::interrupts::ticks();
                println!("System Uptime: {} seconds ({} ticks)", secs, ticks);
            }
            "history" => {
                println!("Command History:");
                for (i, past_cmd) in self.history.iter().enumerate() {
                    println!("  {}: {}", i + 1, past_cmd);
                }
            }
            _ => println!("Unknown command: '{}'. Type 'help' for available commands.", cmd),
        }
    }
}