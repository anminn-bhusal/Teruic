//C Foreign Function Interface (FFI) and System Call Dispatcher 

use core::ffi::c_char;
use crate::vfs::VFS;

/// Standard C string length helper
#[no_mangle]
pub unsafe extern "C" fn teruic_strlen(s: *const c_char) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

/// Print C string directly to kernel output buffer
#[no_mangle]
pub unsafe extern "C" fn teruic_print_c_str(s: *const c_char) {
    let len = teruic_strlen(s);
    let slice = core::slice::from_raw_parts(s as *const u8, len);
    if let Ok(text) = core::str::from_utf8(slice) {
        crate::print!("{}", text);
    }
}

/// C API for reading files from Teruic VFS
#[no_mangle]
pub unsafe extern "C" fn teruic_read_file(
    filename: *const c_char,
    out_buf: *mut u8,
    max_len: usize,
) -> i32 {
    let len = teruic_strlen(filename);
    let slice = core::slice::from_raw_parts(filename as *const u8, len);
    
    if let Ok(name) = core::str::from_utf8(slice) {
        if let Some(bytes) = VFS.lock().read_file(name) {
            let copy_len = core::cmp::min(bytes.len(), max_len);
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, copy_len);
            return copy_len as i32;
        }
    }
    -1 // Error / File Not Found
}