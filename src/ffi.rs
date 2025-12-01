use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::interpreter::Interpreter;

#[no_mangle]
pub extern "C" fn run_dr_script(code: *const c_char) -> *mut c_char {
    if code.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(code) };
    let code_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let mut interpreter = Interpreter::new();
    let result = match interpreter.run(code_str) {
        Ok(output) => output,
        Err(e) => format!("Error: {}", e),
    };

    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn free_dr_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}