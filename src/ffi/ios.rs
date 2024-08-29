use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};

use raw_window_handle::RawWindowHandle;

use crate::events::ffi::FfiEvent;

use super::{Ffi, FfiAd, FfiGreet, FfiKv, SENDER};

extern "C" {
    fn ffi_kv_get(key: *const c_char) -> *const c_char;
    fn ffi_kv_set(key: *const c_char, val: *const c_char);
    fn ffi_greet_to_swift();
    fn ffi_show_ad();
    fn ffi_ad_init();
    fn ffi_rwh_test(vc: *mut c_void);
}

#[no_mangle]
pub extern "C" fn ffi_greet_to_rust() {
    SENDER.get().unwrap().send(FfiEvent::Greet);
}

#[no_mangle]
pub extern "C" fn ffi_ad_dismiss() {
    SENDER.get().unwrap().send(FfiEvent::AdDismiss)
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

impl FfiGreet for Ffi {
    fn greet() {
        unsafe { ffi_greet_to_swift() }
    }
}

impl Ffi {
    pub fn init_ad() {
        unsafe { ffi_ad_init() };
    }
}

impl Ffi {
    pub fn rwh_test(rwh: RawWindowHandle) {
        if let RawWindowHandle::UiKit(uikit) = rwh {
            let vc = uikit.ui_view_controller.unwrap();
            unsafe {
                ffi_rwh_test(vc.as_ptr());
            }
        }
    }
}

impl FfiAd for Ffi {
    fn show() {
        unsafe { ffi_show_ad() }
    }
}
