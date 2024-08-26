use bevy::prelude::*;

use crate::systems;

pub struct MaskPlugin;

impl Plugin for MaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::mask::spawn_mask);
    }
}
