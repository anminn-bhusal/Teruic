// For now this is just in testing phase for shell
// as this is kernel lvl shell so it may be temporary or not as well so idk the future.

// Todo: many things i even dont know but at first creating commands for further development


use alloc::string::String;
use alloc::vec::Vec;
use crate::{print, println};
use crate::vfs::VFS;
use crate::gui::UI;
use x86_64::instructions::port::Port;


/// Initiates an x86 ACPI/QEMU soft shutdown
pub fn shutdown_system() -> ! {
    crate::println!("\n[Teruic OS] Shutting down kernel...");
    
    // Send QEMU ACPI shutdown signal
    unsafe {
        let mut port = Port::new(0x604);
        port.write(0x2000u16);
        
        // Secondary fallback port for BOCHS/QEMU
        let mut fallback_port = Port::new(0xB004);
        fallback_port.write(0x2000u16);
    }

    // Halt loop if power off is delayed
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
    
    /// Execute a single string command line
    pub fn run_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return; // Ignore empty lines and script comments
        }

        let args: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = args[0];

        match cmd {
            "help" => {
                println!("Teruic OS Shell Commands:");
                println!("  ls             - List files in Virtual File System");
                println!("  cat <file>     - Display contents of a file");
                println!("  write <file>   - Create a file (e.g. write script.sh echo hello)");
                println!("  exec <file>    - Execute a shell script or program file");
                println!("  c_app          - Run sample C application window");
                println!("  clear          - Clear terminal screen");
                println!("  info           - System hardware & kernel info");
                println!("  uptime         - System uptime in seconds");
                println!("  java <file>    - Run Java bytecode/program using Embedded JVM");
                println!("  c_run <file>   - Run C source code program using C Runtime");
                println!("  shutdown       - Power off the system safely");
            }
            "c_run" => {
                if args.len() > 1 {
                    let filename = args[1];
                    crate::c_runner::CRunner::execute_file(filename);
                } else {
                    println!("Usage: c_run <filename>");
                }
            }
            "shutdown" => {
                shutdown_system();
            }
            "edit" => {
                if args.len() > 1 {
                let filename = args[1];
                unsafe {
                    crate::editor::EDITOR.open(filename);
                }
            } else {
                println!("Usage: edit <filename>");
            }
            }
            "java" => {
            if args.len() > 1 {
            let filename = args[1];
            crate::java::JVM::execute_file(filename);
            } else {
                println!("Usage: java <filename>");
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
            "exec" => {
                if args.len() > 1 {
                    let filename = args[1];
                    self.execute_script(filename);
                } else {
                    println!("Usage: exec <script_filename>");
                }
            }
            "c_app" => {
                UI::draw_window("C Application Bridge", &[
                    "Running native C binary through FFI...",
                    "Accessing VFS and VGA buffers directly.",
                    "Status: Execution Completed Successfully."
                ]);
            }
            "clear" => crate::vga::clear_screen(),
            "info" => {
                println!("Teruic Kernel v0.1.0 (x86_64 Bare-Metal)");
                println!("Shell Interpreter Engine: ACTIVE");
                println!("C FFI & GUI System: ACTIVE");
            }
            "uptime" => {
                let secs = crate::interrupts::uptime_seconds();
                let ticks = crate::interrupts::ticks();
                println!("System Uptime: {} seconds ({} ticks)", secs, ticks);
            }
            _ => println!("Unknown command: '{}'. Type 'help' for commands.", cmd),
        }
    }

    /// Read a script file from VFS and execute line by line
    fn execute_script(&mut self, filename: &str) {
        match VFS.lock().read_file(filename) {
            Some(bytes) => {
                if let Ok(script_content) = core::str::from_utf8(&bytes) {
                    println!("[Exec] Running script '{}'...", filename);
                    for line in script_content.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() && !trimmed.starts_with('#') {
                            println!(">> {}", trimmed);
                            self.run_line(trimmed);
                        }
                    }
                    println!("[Exec] Script execution finished.");
                } else {
                    println!("[Exec Error] Cannot execute binary or non-UTF8 file.");
                }
            }
            None => println!("[Exec Error] Script file '{}' not found in VFS.", filename),
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