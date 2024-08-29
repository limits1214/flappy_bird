mod app;
mod assets;
mod ffi;
mod game;
#[cfg(feature = "inspector")]
mod inspector;
mod main_menu;
mod mask;
mod picking_callback;
mod resize;
mod tween_callback;

pub use app::AppPlugin;
