use wasm_bindgen::prelude::*;
use dr_script::run_source;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run_dr_script(code: &str) -> String {
    run_source(code)
}