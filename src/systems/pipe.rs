use bevy::prelude::*;
use rand::Rng;

use crate::{components::pipe::{Pipe, PipeParent, PipePoint, PointEarned}, constant::Z_INDEX_1, resources::{assets::FlappyBirdAssets, pipe_spawn_timer::PipeSpawnTimer}};

// pub fn pipe_spawn(
//     mut commands: Commands,
//     fb_assets: Res<FlappyBirdAssets>,
// ){
    // let pipe_bottom = fb_assets.pipe_green_bottom.clone();
    // let pipe_top = fb_assets.pipe_green_top.clone();

    // commands.spawn(
    //     (
    //         Name::new("pipe_top"),
    //         SpriteBundle {
    //             texture: pipe_top,
    //             transform: Transform {
    //                 translation: Vec3::new(0., 0., Z_INDEX_1),
    //                 ..default()
    //             },
    //             ..default()
    //         }
    //     )
    // );

    

    // commands.spawn(
    //     (
    //         Name::new("pipe_bottom"),
    //         SpriteBundle {
    //             texture: pipe_bottom,
    //             transform: Transform {
    //                 translation: Vec3::new(0., 0., Z_INDEX_1),
    //                 ..default()
    //             },
    //             ..default()
    //         }
    //     )
    // );
// }

// pub fn pipe_spawn2(
//     time: Res<Time>,
//     mut pipe_spawn_timer: ResMut<PipeSpawnTimer>,

// ){
//     pipe_spawn_timer.0.tick(time.delta());
//     if pipe_spawn_timer.0.just_finished() {
//         info!("pipe spawn");
//     }
// }

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