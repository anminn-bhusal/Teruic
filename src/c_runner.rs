// runtime environment for c

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
                        &["C Runtime Environment v1.0 initialized.", "Executing main()..."]
                    );
                    crate::println!("[C Runtime] Compiling & Executing '{}'...", filename);
                    Self::interpret(code);
                } else {
                    crate::println!("[C Runtime Error] File is not valid ASCII/UTF-8 C source.");
                }
            }
            None => crate::println!("[C Runtime Error] C File '{}' not found in VFS.", filename),
        }
    }

    /// Parse and execute simple C statements, loops, and printf bindings
    fn interpret(source: &str) {
        let mut variables: Vec<(String, i32)> = Vec::new();

        // Standardize lines: ignore empty lines, includes, and function signatures
        let lines: Vec<&str> = source
            .lines()
            .map(|l| l.trim())
            .filter(|l| {
                !l.is_empty()
                    && !l.starts_with("#include")
                    && !l.starts_with("//")
                    && !l.starts_with("int main")
                    && *l != "{"
                    && *l != "}"
            })
            .collect();

        let mut idx = 0;
        while idx < lines.len() {
            let line = lines[idx];

            // 1. Handle C `for` loops e.g. for(int i=0; i<5; i++)
            if line.starts_with("for") {
                if let Some(start) = line.find('(') {
                    if let Some(end) = line.rfind(')') {
                        let header = &line[start + 1..end];
                        let parts: Vec<&str> = header.split(';').collect();

                        if parts.len() == 3 {
                            // Parse init (int i = 0)
                            let init_parts: Vec<&str> = parts[0].split_whitespace().collect();
                            let mut loop_var = String::from("i");
                            let mut start_val = 0;

                            if init_parts.len() >= 4 && init_parts[0] == "int" {
                                loop_var = String::from(init_parts[1]);
                                start_val = init_parts[3].parse::<i32>().unwrap_or(0);
                            }

                            // Parse condition limit (i < 10 or i <= 10)
                            let cond_parts: Vec<&str> = parts[1].split_whitespace().collect();
                            let mut end_val = 10;
                            if cond_parts.len() >= 3 {
                                end_val = cond_parts[2].parse::<i32>().unwrap_or(10);
                            }

                            // Collect body inside loop (next line if non-braced or until })
                            let mut body_line = "";
                            if idx + 1 < lines.len() {
                                body_line = lines[idx + 1];
                                idx += 1; // consume body line
                            }

                            // Execute loop
                            for val in start_val..=end_val {
                                Self::update_variable(&mut variables, &loop_var, val);
                                Self::execute_single_line(body_line, &mut variables);
                            }
                        }
                    }
                }
            } 
            // 2. Handle standard single C statements
            else {
                Self::execute_single_line(line, &mut variables);
            }

            idx += 1;
        }

        crate::println!("[C Runtime] Program execution finished successfully.\n");
    }

    fn update_variable(vars: &mut Vec<(String, i32)>, name: &str, val: i32) {
        for (n, v) in vars.iter_mut() {
            if n == name {
                *v = val;
                return;
            }
        }
        vars.push((String::from(name), val));
    }

    fn execute_single_line(line: &str, variables: &mut Vec<(String, i32)>) {
        let trimmed = line.trim();

        // Handle printf(...)
        if trimmed.starts_with("printf(") {
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.rfind(')') {
                    let content = &trimmed[start + 1..end];

                    if content.contains(',') {
                        let parts: Vec<&str> = content.splitn(2, ',').collect();
                        let fmt_str = parts[0].trim().trim_matches('"');
                        let var_name = parts[1].trim().trim_end_matches(';').trim();

                        let mut val = 0;
                        for (name, v) in variables.iter() {
                            if name == var_name {
                                val = *v;
                                break;
                            }
                        }

                        let out_str = fmt_str.replace("%d", &format!("{}", val)).replace("\\n", "");
                        crate::println!("{}", out_str);
                    } else {
                        let clean_str = content.trim().trim_matches('"').replace("\\n", "");
                        crate::println!("{}", clean_str);
                    }
                }
            }
        }
        // Handle variable creation (e.g. int result = 42;)
        else if trimmed.starts_with("int ") {
            let stmt = trimmed.trim_end_matches(';');
            let parts: Vec<&str> = stmt.split_whitespace().collect();
            if parts.len() >= 4 && parts[2] == "=" {
                let var_name = String::from(parts[1]);
                if let Ok(val) = parts[3].parse::<i32>() {
                    Self::update_variable(variables, &var_name, val);
                }
            }
        }
    }
}