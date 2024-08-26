use serde::{Deserialize, Serialize};

#[cfg(not(any(target_os = "ios", target_os = "android", target_arch = "wasm32")))]
mod desktop;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "android")]
mod android;

#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    pub score: u32,
}

pub trait FfiKv {
    fn get(key: &str) -> String;
    fn set(key: &str, val: &str);
}

pub struct Ffi;
