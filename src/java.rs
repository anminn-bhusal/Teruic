// creating embedded jvm engine 
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::vfs::VFS;
use crate::gui::UI;

/// Simplified Java Stack Frame
pub struct JavaFrame {
    pub operand_stack: Vec<i32>,
    pub local_variables: [i32; 8],
}

impl JavaFrame {
    pub fn new() -> Self {
        Self {
            operand_stack: Vec::new(),
            local_variables: [0; 8],
        }
    }
}

pub struct JVM;

impl JVM {
    /// Executes a text file containing simple Java instructions or bytecode representations
    pub fn execute_file(filename: &str) {
        match VFS.lock().read_file(filename) {
            Some(bytes) => {
                if let Ok(code) = core::str::from_utf8(&bytes) {
                    UI::draw_window(
                        &format!("Java Virtual Machine - [{}]", filename),
                        &["JVM Runtime Environment v1.0 initialized.", "Executing Java main()..."]
                    );
                    Self::interpret(code);
                } else {
                    crate::println!("[JVM Error] File is not valid text/bytecode source.");
                }
            }
            None => crate::println!("[JVM Error] Class/File '{}' not found in VFS.", filename),
        }
    }

    /// Basic Bytecode Interpreter Routine
    fn interpret(source: &str) {
        let mut frame = JavaFrame::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let opcode = parts[0];

            match opcode {
                "println" | "System.out.println" => {
                    let msg = if parts.len() > 1 {
                        parts[1..].join(" ")
                    } else if let Some(val) = frame.operand_stack.pop() {
                        format!("{}", val)
                    } else {
                        String::new()
                    };
                    let clean_msg = msg.trim_matches('"');
                    crate::println!("[Java stdout] {}", clean_msg);
                }
                "bipush" | "iconst" => {
                    if parts.len() > 1 {
                        if let Ok(val) = parts[1].parse::<i32>() {
                            frame.operand_stack.push(val);
                        }
                    }
                }
                "iadd" => {
                    if frame.operand_stack.len() >= 2 {
                        let b = frame.operand_stack.pop().unwrap();
                        let a = frame.operand_stack.pop().unwrap();
                        frame.operand_stack.push(a + b);
                    }
                }
                "imul" => {
                    if frame.operand_stack.len() >= 2 {
                        let b = frame.operand_stack.pop().unwrap();
                        let a = frame.operand_stack.pop().unwrap();
                        frame.operand_stack.push(a * b);
                    }
                }
                "istore" => {
                    if parts.len() > 1 {
                        if let Ok(idx) = parts[1].parse::<usize>() {
                            if idx < 8 {
                                if let Some(val) = frame.operand_stack.pop() {
                                    frame.local_variables[idx] = val;
                                }
                            }
                        }
                    }
                }
                "iload" => {
                    if parts.len() > 1 {
                        if let Ok(idx) = parts[1].parse::<usize>() {
                            if idx < 8 {
                                frame.operand_stack.push(frame.local_variables[idx]);
                            }
                        }
                    }
                }
                "return" => break,
                _ => {
                    if trimmed.contains("System.out.println") {
                        if let Some(start) = trimmed.find('(') {
                            if let Some(end) = trimmed.rfind(')') {
                                let content = &trimmed[start + 1..end];
                                crate::println!("[Java stdout] {}", content.trim_matches('"'));
                            }
                        }
                    }
                }
            }
        }
        crate::println!("[JVM] Execution finished successfully.\n");
    }
}