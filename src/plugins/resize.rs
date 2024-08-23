use bevy::prelude::*;

use crate::{events::resize::ResizeEvent, systems};

pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ResizeEvent>()
            .add_systems(Update, systems::resize::resize);
    }
}