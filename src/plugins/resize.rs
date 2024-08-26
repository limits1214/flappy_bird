use bevy::{prelude::*, window::WindowResized};

use crate::{events::resize::ResizeEvent, systems};

pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResizeEvent>().add_systems(
            Update,
            systems::resize::resize
                .run_if(on_event::<WindowResized>().or_else(on_event::<ResizeEvent>())),
        );
    }
}
