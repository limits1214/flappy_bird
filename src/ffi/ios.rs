use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use super::{Ffi, FfiKv};

extern "C" {
    fn ffi_kv_get(key: *const c_char) -> *const c_char;
    fn ffi_kv_set(key: *const c_char, val: *const c_char);
}

impl FfiKv for Ffi {
    fn get(key: &str) -> String {
        let key = CString::new(key).unwrap();
        unsafe {
            let ptr = ffi_kv_get(key.into_raw());
            let c_str = CStr::from_ptr(ptr);
            match c_str.to_str() {
                Ok(val) => String::from(val),
                Err(_) => String::new(),
            }
        }
    }

    fn set(key: &str, val: &str) {
        let key = CString::new(key).unwrap();
        let val = CString::new(val).unwrap();
        unsafe { ffi_kv_set(key.into_raw(), val.into_raw()) };
    }
}
