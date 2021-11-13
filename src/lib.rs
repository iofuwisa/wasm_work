extern crate serde_json;
extern crate serde;
extern crate wasm_bindgen;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use js_sys::{Uint8Array, Int32Array};

#[macro_use]
extern crate serde_derive;


#[derive(Serialize, Deserialize)]
pub struct Arr {
   pub value: Vec<u32>,
}


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

#[wasm_bindgen]
pub fn array_js2rust(js_object: &JsValue) {
    // let arr: Arr = js_object.into_serde().unwrap();
    // console_log(&format!("{:?}", arr.value));
    // let arr: Vec<u8> = js_object.into_serde().unwrap();
    // console_log(&format!("{:?}", arr));

    let array = Uint8Array::new(&js_object);
    let arr: Vec<u8> = array.to_vec();
    console_log(&format!("{:?}", arr));
}

#[wasm_bindgen]
pub fn array_rust2js() -> js_sys::Int32Array {
    let array: [i32;5] = [5, 6, 7, 8, 9];
    return Int32Array::from(&array[..]);
}

#[wasm_bindgen(start)]
pub fn run() {
    console_log("start!!!!!!!!!!!!!!!!!");
}