use bevy::prelude::*;

pub trait ColorExtension {
    fn new_alpha_0() -> Self;
}

impl ColorExtension for Color {
    fn new_alpha_0() -> Self {
        Self::srgba_u8(0, 0, 0, 0)
    }
}

pub trait SpriteExtension {
    fn alpah_0_sprite() -> Self;
}

impl SpriteExtension for Sprite {
    fn alpah_0_sprite() -> Self {
        Self {
            color: Color::new_alpha_0(),
            ..default()
        }
    }
}
