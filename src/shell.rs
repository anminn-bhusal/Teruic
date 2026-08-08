// For now this is just in testing phase for shell
// as this is kernel lvl shell so it may be temporary or not as well so idk the future.

use alloc::string::String;
use alloc::vec::Vec;
use crate::c_runner::CRunner;
use crate::{print, println};
use crate::vfs::VFS;
use x86_64::instructions::port::Port;
use spin::Mutex;

pub static SHELL: Mutex<Shell> = Mutex::new(Shell::new());

pub fn shutdown_system() -> ! {
    crate::println!("\n[Teruic OS] Shutting down kernel...");
    unsafe {
        let mut port = Port::new(0x604);
        port.write(0x2000u16);
    }
    loop {
        x86_64::instructions::hlt();
    }
}

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
            '\n' | '\r' => {
                println!();
                self.execute_command();
                self.buffer.clear();
                print!("teruic> ");
            }
            '\x08' | '\x7f' => {
                if self.buffer.pop().is_some() {
                    print!("\x08 \x08"); 
                }
            }
            character if character >= ' ' && character <= '~' => {
                self.buffer.push(character);
                print!("{}", character);
            }
            _ => {} 
        }
    }
    
    pub fn run_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return;
        }

        let args: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = args[0];

        match cmd {
            "help" => {
                println!("Teruic OS Shell Commands:");
                println!("  ls             - List files in Virtual File System");
                println!("  cat <file>     - Display contents of a file");
                println!("  write <file>   - Create a file");
                println!("  edit <file>    - Open file in TeruicPad full-screen editor");
                println!("  c_run <file>   - Execute a C program file");
                println!("  clear          - Clear terminal screen");
                println!("  info           - System hardware & kernel info");
                println!("  uptime         - System uptime in seconds");
                println!("  shutdown       - Power off the system safely");
                println!("  runbin <file>  - Execute native x86_64 assembly/C machine code binary");
            }
            "shutdown" => {
                shutdown_system();
            }
            "edit" => {
                if args.len() > 1 {
                    let filename = args[1];
                    crate::editor::EDITOR.lock().open(filename);
                } else {
                    println!("Usage: edit <filename>");
                }
            }
            "hexwrite" => {
    if args.len() > 2 {
        let filename = args[1];
        let mut bytes = Vec::new();

        for hex_str in &args[2..] {
            if let Ok(byte) = u8::from_str_radix(hex_str, 16) {
                bytes.push(byte);
            } else {
                println!("Invalid hex byte: '{}'", hex_str);
                return;
            }
        }

        VFS.lock().write_file(filename, bytes);
        println!("Successfully wrote {} bytes to '{}'", args.len() - 2, filename);
    } else {
        println!("Usage: hexwrite <filename> <hex_bytes...>");
        println!("Example: hexwrite test.bin 90 90 c3");
    }
}
            "c_run" => {
                if args.len() > 1 {
                    let filename = args[1];
                    CRunner::execute_file(filename);
                } else {
                    println!("Usage: c_run <filename>");
                }
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
                    println!("Saved to VFS file: '{}'", filename);
                } else {
                    println!("Usage: write <filename> <content>");
                }
            }
            "clear" => crate::vga::clear_screen(),
            "info" => {
                println!("Teruic Kernel v0.1.0 (x86_64 Bare-Metal)");
                println!("C Runtime & Shell: ACTIVE");
            }
            "runbin" => {
                if args.len() > 1 {
                    let filename = args[1];
                    crate::loader::NativeLoader::execute_binary(filename);
                } else {
                    println!("Usage: runbin <binary_filename>");
                }
            }
            "uptime" => {
                let secs = crate::interrupts::uptime_seconds();
                println!("System Uptime: {} seconds", secs);
            }
            _ => println!("Unknown command: '{}'. Type 'help' for commands.", cmd),
        }
    }

    fn execute_command(&mut self) {
        if self.buffer.trim().is_empty() {
            return;
        }
        let line = self.buffer.clone();
        self.history.push(line.clone());
        self.run_line(&line);
    }
}