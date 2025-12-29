use crate::io::DrScriptIo;
use lazy_static::lazy_static;
use std::ffi::{CStr, CString, c_char};
use std::sync::{Condvar, Mutex};

lazy_static! {
    static ref INPUT_CHANELL: (Mutex<Option<String>>, Condvar) = (Mutex::new(None), Condvar::new());
}

#[unsafe(no_mangle)]
pub extern "C" fn submit_input(text: *const c_char) {
    if text.is_null() {
        return;
    }
    let c_str = unsafe { CStr::from_ptr(text) };
    let rust_str = c_str.to_string_lossy().into_owned();
    let (lock, cvar) = &*INPUT_CHANELL;
    let mut starter = lock.lock().unwrap();
    *starter = Some(rust_str);
    cvar.notify_one();
}

#[allow(dead_code)]
pub type PrintCallback = extern "C" fn(*const c_char);
#[allow(dead_code)]
pub type InputCallback = extern "C" fn(*const c_char);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IoFlutter {
    print_c: PrintCallback,
    input_c: InputCallback,
}

impl IoFlutter {
    #[allow(dead_code)]
    pub fn new(print_c: PrintCallback, input_c: InputCallback) -> Self {
        Self { print_c, input_c }
    }
}

impl DrScriptIo for IoFlutter {
    fn print(&mut self, message: &str) {
        let c_msg = CString::new(message)
            .unwrap_or_else(|_| CString::new("Error encoding string").unwrap());
        (self.print_c)(c_msg.as_ptr());
    }
    fn input(&mut self, prompt: &str) -> Result<String, String> {
        let c_prompt =
            CString::new(prompt).unwrap_or_else(|_| CString::new("Error encoding string").unwrap());
        (self.input_c)(c_prompt.as_ptr());

        let (lock, cvar) = &*INPUT_CHANELL;

        // Polling с таймаутом вместо бесконечного ожидания
        loop {
            let mut input_val = lock.lock().unwrap();

            if let Some(value) = input_val.take() {
                return Ok(value);
            }

            // Ждем 50ms и проверяем снова
            let result = cvar
                .wait_timeout(input_val, std::time::Duration::from_millis(50))
                .unwrap();
            input_val = result.0;

            if let Some(value) = input_val.take() {
                return Ok(value);
            }
        }
    }
}
