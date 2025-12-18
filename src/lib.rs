use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod ast;
mod builtins;
mod interpreter;
mod io;
mod lexer;
mod parser;
mod type_error;
mod type_values;

use crate::interpreter::Interpretation;
use crate::lexer::lex;
use crate::parser::ParserToken;

#[unsafe(no_mangle)]
pub extern "C" fn run_dr_script(code: *const c_char) -> *mut c_char {
    if code.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(code) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = execute_dr_code(rust_str);

    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_dr_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

fn execute_dr_code(code: &str) -> String {
    let tokens = lex(code);

    let mut parser = ParserToken::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => return e.get_report_string(),
    };
    "lsj".to_string()

    // let mut interpreter = Interpretation::new(None);
    // interpreter
    //     .run(ast).map_err(|e| e.get_report_string()).unwrap_or_else(|e| format!("Error: {}", e))
}
