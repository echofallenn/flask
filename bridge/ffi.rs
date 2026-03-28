/// FFI layer — exposes Rust functions as C-compatible symbols
/// so the C++ resolver can call back into Rust if needed.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Returns 1 if a binary exists in PATH, 0 otherwise.
/// Called from C++ to check backend availability.
#[no_mangle]
pub extern "C" fn flask_which(bin: *const c_char) -> i32 {
    if bin.is_null() { return 0; }
    let name = unsafe { CStr::from_ptr(bin) }.to_string_lossy();
    let found = std::process::Command::new("which")
        .arg(name.as_ref())
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if found { 1 } else { 0 }
}

/// Frees a CString allocated by Rust and passed to C++.
#[no_mangle]
pub extern "C" fn flask_free_string(ptr: *mut c_char) {
    if ptr.is_null() { return; }
    unsafe { drop(CString::from_raw(ptr)); }
}
