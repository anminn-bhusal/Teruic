// visual file system

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

    /// Pre-loads essential directories and welcome files into the VFS
    pub fn init(&mut self) {
    let bin = BTreeMap::new();
    let apps = BTreeMap::new();

    self.root.insert(
        "hello.txt".to_string(),
        Node::File(b"Welcome to Teruic OS File System!".to_vec()),
    );

    // Create a sample shell script in the VFS
    let sample_script = b"# Teruic OS Automation Script\ninfo\nuptime\nwrite boot.log System automated boot test\ncat boot.log\n";
    self.root.insert(
        "demo.sh".to_string(),
        Node::File(sample_script.to_vec()),
    );

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

    /// Read file content as bytes
    pub fn read_file(&self, name: &str) -> Option<Vec<u8>> {
        match self.root.get(name) {
            Some(Node::File(data)) => Some(data.clone()),
            _ => None,
        }
    }
}