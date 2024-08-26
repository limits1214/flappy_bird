use crate::components::prelude::*;
use bevy::prelude::*;

pub fn ground_animation(time: Res<Time>, mut q_ground: Query<&mut Transform, With<Ground>>) {
    if let Ok(mut transform) = q_ground.get_single_mut() {
        transform.translation.x -= 50. * time.delta_seconds();

        if transform.translation.x <= -12. {
            transform.translation.x = 12.;
        }
    }
}
