pub mod mask;
pub mod resize;
pub mod bird;
pub mod states;
pub mod ground;
pub mod main_menu;
pub mod button;
pub mod guide;
pub mod puase;
pub mod pipe;
pub mod score;
pub mod result;
pub mod sparkle;

use bevy::prelude::*;

#[derive(Component)]
pub struct Bg;