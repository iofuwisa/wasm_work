extern crate serde_json;
extern crate serde;
extern crate wasm_bindgen;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use js_sys::Uint8Array;

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
    console_log(&format!("{:?}", js_object));
    // let arr: Arr = js_object.into_serde().unwrap();
    // console_log(&format!("{:?}", arr.value));
    // let arr: Vec<u8> = js_object.into_serde().unwrap();
    // console_log(&format!("{:?}", arr));

    let array = Uint8Array::new(&js_object);
    let arr: Vec<u8> = array.to_vec();
    console_log(&format!("{:?}", arr));
}