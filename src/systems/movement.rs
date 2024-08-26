use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::prelude::Title,
    prelude::{Ground, PipeParent, PipePoint, PointEarned},
};

pub fn title_movement(time: Res<Time>, mut q_title: Query<&mut Transform, With<Title>>) {
    if let Ok(mut transform) = q_title.get_single_mut() {
        transform.translation.y = 60. + (time.elapsed_seconds() * 2.).sin() * 2.;
    }
}

pub fn ground_movement(time: Res<Time>, mut q_ground: Query<&mut Transform, With<Ground>>) {
    if let Ok(mut transform) = q_ground.get_single_mut() {
        transform.translation.x -= 50. * time.delta_seconds();

        if transform.translation.x <= -12. {
            transform.translation.x = 12.;
        }
    }
}

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
