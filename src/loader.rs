// Native Binary Loader for Teruic OS

pub struct NativeLoader;

impl NativeLoader {
    pub fn execute_binary(filename: &str) {
        if let Some(bytes) = crate::vfs::VFS.lock().read_file(filename) {
            crate::println!("[NativeLoader] Executing raw binary slice ({} bytes)...", bytes.len());

            // Read Time Stamp Counter before execution using x86_64 CPU intrinsic
            let start_tsc = unsafe { core::arch::x86_64::_rdtsc() };

            let func: extern "C" fn() = unsafe { core::mem::transmute(bytes.as_ptr()) };
            func();

            // Read Time Stamp Counter after execution
            let end_tsc = unsafe { core::arch::x86_64::_rdtsc() };

            crate::println!(
                "[NativeLoader] Execution finished in {} CPU cycles.",
                end_tsc.saturating_sub(start_tsc)
            );
        } else {
            crate::println!("[NativeLoader] Error: Binary file '{}' not found.", filename);
        }
    }
}