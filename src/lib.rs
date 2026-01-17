use crate::ffi_io::{InputCallback, IoFlutter, PrintCallback};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::rc::Rc;

mod ast;
mod ffi_io;
mod functions;
mod interpreter;
mod io;
mod lexer;
mod parser;
mod type_error;
mod type_values;
mod import;


use crate::interpreter::Interpretation;
use crate::io::DrScriptIo;
use crate::lexer::lex;
use crate::parser::ParserToken;

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_dr_script(
    code: *const c_char,
    print_c: PrintCallback,
    input_c: InputCallback,
) {
    if code.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(code) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            let e = CString::new("Error: Invalid UTF-8 code").unwrap();
            (print_c)(e.as_ptr());
            return;
        }
    };

    let flutter_io = IoFlutter::new(print_c, input_c);
    let io_ref = Rc::new(RefCell::new(flutter_io));
    execute_dr_code(&rust_str, io_ref);
}
/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_dr_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

fn execute_dr_code(code: &str, io: Rc<RefCell<dyn DrScriptIo>>) {
    let tokens = lex(code);
    let mut parser = ParserToken::new(tokens);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            io.borrow_mut().print(&format!("Parse Error: {}", e));
            return;
        }
    };

    let mut interpreter = Interpretation::new(Some(io.clone()));
    if let Err(e) = interpreter.run(ast) {
        io.borrow_mut().print(&format!("Runtime Error: {:?}", e));
    }
}
