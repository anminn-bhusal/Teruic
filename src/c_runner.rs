// c interpreter and runtime environment

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::vfs::VFS;
use crate::gui::UI;

pub struct CRunner;

impl CRunner {
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

    fn interpret(source: &str) {
        let mut variables: Vec<(String, i32)> = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() 
                || trimmed.starts_with("#include") 
                || trimmed.starts_with("//") 
                || trimmed.starts_with("int main") 
                || trimmed.starts_with("void main")
                || trimmed == "{" 
                || trimmed == "}" 
            {
                continue;
            }

            // Parse printf statements
            if trimmed.starts_with("printf(") {
                if let Some(start) = trimmed.find('(') {
                    if let Some(end) = trimmed.rfind(')') {
                        let content = &trimmed[start + 1..end];
                        let parts: Vec<&str> = content.splitn(2, ',').collect();
                        let clean_str = parts[0].trim().trim_matches('"');

                        if parts.len() > 1 && clean_str.contains("%d") {
                            let var_expr = parts[1].trim().trim_end_matches(';');
                            let mut val = 0;
                            
                            // Check if it's a literal or variable name
                            if let Ok(lit) = var_expr.parse::<i32>() {
                                val = lit;
                            } else {
                                for (name, v) in &variables {
                                    if name == var_expr {
                                        val = *v;
                                        break;
                                    }
                                }
                            }
                            let fmt_out = clean_str.replace("%d", &format!("{}", val));
                            crate::println!("{}", fmt_out);
                        } else {
                            crate::println!("{}", clean_str);
                        }
                    }
                }
            }
            // Parse variable declarations (e.g., int x = 10; or int x = y + 5;)
            else if trimmed.starts_with("int ") {
                let stmt = trimmed.trim_end_matches(';').trim();
                let parts: Vec<&str> = stmt.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let declaration_parts: Vec<&str> = parts[0].split_whitespace().collect();
                    if declaration_parts.len() >= 2 {
                        let var_name = String::from(declaration_parts[1]);
                        let rhs = parts[1];

                        // Evaluate RHS value
                        let mut val = 0;
                        if let Ok(parsed_val) = rhs.parse::<i32>() {
                            val = parsed_val;
                        } else {
                            // Simple variable lookup fallback
                            for (name, v) in &variables {
                                if name == rhs {
                                    val = *v;
                                    break;
                                }
                            }
                        }

                        // Update or push variable
                        if let Some(existing) = variables.iter_mut().find(|(n, _)| *n == var_name) {
                            existing.1 = val;
                        } else {
                            variables.push((var_name, val));
                        }
                    }
                }
            }
            // Variable reassignment (e.g., x = 20;)
            else if trimmed.contains('=') && !trimmed.starts_with("==") {
                let stmt = trimmed.trim_end_matches(';').trim();
                let parts: Vec<&str> = stmt.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let var_name = parts[0];
                    let rhs = parts[1];
                    let mut val = 0;
                    if let Ok(parsed_val) = rhs.parse::<i32>() {
                        val = parsed_val;
                    } else {
                        for (name, v) in &variables {
                            if name == rhs {
                                val = *v;
                                break;
                            }
                        }
                    }
                    if let Some(existing) = variables.iter_mut().find(|(n, _)| n == var_name) {
                        existing.1 = val;
                    }
                }
            }
            else if trimmed.starts_with("return") {
                break;
            }
        }
        crate::println!("[C Runtime] Execution completed successfully.\n");
    }
}