use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::game::{PipeParent, PipePoint, PointEarned},
    constant::Z_INDEX_1,
    resources::assets::FlappyBirdAssets,
};

pub fn pipe_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut q_pipe: Query<(&mut Transform, &Children), With<PipeParent>>,
    q_pipe_point: Query<Entity, With<PipePoint>>,
) {
    for (mut transform, children) in &mut q_pipe {
        transform.translation.x += -50. * time.delta_seconds();
        if transform.translation.x <= -85. {
            let r = rand::thread_rng().gen_range((-30.0)..(100.0));
            transform.translation.x = 85.;
            transform.translation.y = r;
            for entity in children.iter() {
                if let Ok(entity) = q_pipe_point.get(*entity) {
                    commands.entity(entity).remove::<PointEarned>();
                }
            }
        }
    }
}
