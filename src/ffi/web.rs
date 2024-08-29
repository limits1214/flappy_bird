use crate::events::ffi::FfiEvent;

use super::{Ffi, FfiGreet, FfiKv, SENDER};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn ffi_kv_get(key: &str) -> JsValue;
    fn ffi_kv_set(key: &str, val: &str);
    fn ffi_greet_to_js();
}

#[wasm_bindgen]
pub fn ffi_greet_to_rust() {
    SENDER.get().unwrap().send(FfiEvent::Greet);
}

impl FfiKv for Ffi {
    fn get(key: &str) -> String {
        ffi_kv_get(key).as_string().unwrap_or_default()
    }

    fn set(key: &str, val: &str) {
        ffi_kv_set(key, val);
    }
}

impl FfiGreet for Ffi {
    fn greet() {
        ffi_greet_to_js();
    }
}
