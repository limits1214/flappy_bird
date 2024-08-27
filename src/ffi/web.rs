use super::{Ffi, FfiKv};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn ffi_kv_get(key: &str) -> JsValue;
    fn ffi_kv_set(key: &str, val: &str);
}

impl FfiKv for Ffi {
    fn get(key: &str) -> String {
        ffi_kv_get(key).as_string().unwrap_or_default()
    }

    fn set(key: &str, val: &str) {
        ffi_kv_set(key, val);
    }
}
