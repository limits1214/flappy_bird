use std::sync::OnceLock;

use bevy_crossbeam_event::CrossbeamEventSender;
use serde::{Deserialize, Serialize};

use crate::events::ffi::FfiEvent;

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

pub trait FfiGreet {
    fn greet();
}

#[cfg(any(target_os = "ios", target_os = "android"))]
pub trait FfiAd {
    fn show();
}

pub struct Ffi;

static SENDER: OnceLock<CrossbeamEventSender<FfiEvent>> = OnceLock::new();
pub fn set_sender(sender: CrossbeamEventSender<FfiEvent>) {
    let _ = SENDER.set(sender);
}
