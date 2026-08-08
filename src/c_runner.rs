// c interpreter and runtime environment

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::vfs::VFS;
use crate::gui::UI;

pub struct CRunner;

impl CRunner {
    /// Read a .c file from VFS and execute it inside the C Runtime Environment
    pub fn execute_file(filename: &str) {
        match VFS.lock().read_file(filename) {
            Some(bytes) => {
                if let Ok(code) = core::str::from_utf8(&bytes) {
                    UI::draw_window(
                        &format!("C Runtime Engine - [{}]", filename),
                        &["C Runtime Environment v1.0 initialized.", "Compiling & executing main()..."]
                    );
                    Self::interpret(code);
                } else {
                    crate::println!("[C Runtime Error] File is not valid ASCII/UTF-8 C source.");
                }
            }
            None => crate::println!("[C Runtime Error] C File '{}' not found in VFS.", filename),
        }
    }

    /// Parse and execute simple C statements line by line
    fn interpret(source: &str) {
        let mut variables: Vec<(String, i32)> = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();

            // Skip empty lines, preprocessor directives, and comments
            if trimmed.is_empty() 
                || trimmed.starts_with("#include") 
                || trimmed.starts_with("//") 
                || trimmed.starts_with("int main") 
                || trimmed == "{" 
                || trimmed == "}" 
            {
                continue;
            }

            // Parse printf(...)
            if trimmed.starts_with("printf(") {
                if let Some(start) = trimmed.find('(') {
                    if let Some(end) = trimmed.rfind(')') {
                        let content = &trimmed[start + 1..end];
                        let clean_str = content.trim().trim_matches('"');
                        
                        // Handle formatting simple %d variables
                        if clean_str.contains("%d") {
                            let parts: Vec<&str> = content.split(',').collect();
                            if parts.len() > 1 {
                                let var_name = parts[1].trim();
                                let mut val = 0;
                                for (name, v) in &variables {
                                    if name == var_name {
                                        val = *v;
                                        break;
                                    }
                                }
                                let fmt_out = clean_str.replace("%d", &format!("{}", val));
                                crate::println!("[C stdout] {}", fmt_out);
                            }
                        } else {
                            crate::println!("[C stdout] {}", clean_str);
                        }
                    }
                }
            }
            // Parse int variable declaration (e.g. int x = 10;)
            else if trimmed.starts_with("int ") {
                let stmt = trimmed.trim_end_matches(';');
                let parts: Vec<&str> = stmt.split_whitespace().collect();
                if parts.len() >= 4 && parts[2] == "=" {
                    let var_name = String::from(parts[1]);
                    if let Ok(val) = parts[3].parse::<i32>() {
                        variables.push((var_name, val));
                    }
                }
            }
            // Parse return 0;
            else if trimmed.starts_with("return") {
                break;
            }
        }
        crate::println!("[C Runtime] Execution completed successfully.\n");
    }
}