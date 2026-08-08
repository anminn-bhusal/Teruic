// virtual file system


use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::Mutex;

/// File types in the VFS
#[derive(Debug, Clone)]
pub enum Node {
    File(Vec<u8>),
    Directory(BTreeMap<String, Node>),
}

/// Thread-safe global VFS root instance
pub static VFS: Mutex<VFSManager> = Mutex::new(VFSManager::new());

pub struct VFSManager {
    root: BTreeMap<String, Node>,
}

impl VFSManager {
    pub const fn new() -> Self {
        Self {
            root: BTreeMap::new(),
        }
    }

    /// Pre-loads essential directories, scripts, and sample programs into the VFS
    pub fn init(&mut self) {
        let bin = BTreeMap::new();
        let apps = BTreeMap::new();

        self.root.insert(
            "hello.txt".to_string(),
            Node::File(b"Welcome to Teruic OS File System!".to_vec()),
        );

        // Pre-load Sample C Program
        let c_sample = b"#include <stdio.h>\n\nint main() {\n    printf(\"Hello from C code running on Teruic OS!\\n\");\n    int result = 42;\n    printf(\"Result code: %d\\n\", result);\n    return 0;\n}\n";
        self.root.insert("main.c".to_string(), Node::File(c_sample.to_vec()));

        // Pre-load Sample Java Program
        let java_sample = b"// Teruic OS Java Application\nSystem.out.println \"Hello from Java running on Teruic OS!\"\nbipush 15\nbipush 27\niadd\nistore 0\niload 0\nprintln\nreturn\n";
        self.root.insert("Main.java".to_string(), Node::File(java_sample.to_vec()));

        // Pre-load Sample Shell Script
        let sample_script = b"# Teruic OS Automation Script\ninfo\nuptime\n";
        self.root.insert("demo.sh".to_string(), Node::File(sample_script.to_vec()));

        self.root.insert("bin".to_string(), Node::Directory(bin));
        self.root.insert("apps".to_string(), Node::Directory(apps));
    }

    /// List all root files and directories
    pub fn list(&self) -> Vec<String> {
        self.root.keys().cloned().collect()
    }

    /// Write a file to the root file system
    pub fn write_file(&mut self, name: &str, contents: Vec<u8>) {
        self.root.insert(name.to_string(), Node::File(contents));
    }

    /// Read file content as a heap-allocated byte vector (Vec<u8>)
    pub fn read_file(&self, name: &str) -> Option<Vec<u8>> {
        match self.root.get(name) {
            Some(Node::File(data)) => Some(data.clone()),
            _ => None,
        }
    }

    /// Delete a file from the virtual file system
    pub fn remove_file(&mut self, name: &str) -> bool {
        if let Some(Node::File(_)) = self.root.get(name) {
            self.root.remove(name);
            true
        } else {
            false
        }
    }
}