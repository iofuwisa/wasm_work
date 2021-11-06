extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

fn console_log(s: &str) {
    web_sys::console::log_1(&JsValue::from(s));
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello wasm!!!, {}!", name));
    console_log("Hello wasm log");
}