use avian2d::prelude::{Gravity, GravityScale, Mass};
use bevy::{prelude::*, window::WindowResized};

use crate::{components::{bird::Bird, resize::Resizable}, constant::{ORIGINAL_HEIGHT, ORIGINAL_WIDTH}, events::resize::ResizeEvent};

pub fn resize(
    q_window: Query<&Window>,
    mut q_resiz: Query<&mut Transform, With<Resizable>>,
    mut er_window_resize: EventReader<WindowResized>,
    mut er_resize: EventReader<ResizeEvent>,
    mut commands: Commands,
) {
    let mut is_resize = false;
    for _ in er_window_resize.read() {
        is_resize = true;
    }
    for _ in er_resize.read() {
        is_resize = true;
    }
    if is_resize {
        if let Ok(window) = q_window.get_single() {
            let scale_x = window.width() / ORIGINAL_WIDTH;
            let scale_y = window.height() / ORIGINAL_HEIGHT;

            let scale = scale_x.min(scale_y);
            for mut transform in &mut q_resiz {
                transform.scale = Vec3::new(scale, scale, transform.scale.z);
            }
            commands.insert_resource(Gravity(Vec2::NEG_Y * 800. * scale));
        }
    }
}