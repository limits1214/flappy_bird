use bevy::prelude::*;

use crate::{components::score::ScoreParent, events::score::ScoreUpEvent, resources::{assets::FlappyBirdAssets, config::GameConfig}};

pub fn score_up(
    mut commands: Commands,
    mut reader: EventReader<ScoreUpEvent>,
    mut config: ResMut<GameConfig>,
    score_parent: Query<(Entity, &Transform, &Children), With<ScoreParent>>,
    fb_assets: Res<FlappyBirdAssets>,
){
    for _ in reader.read() {
        config.score += 1;
        
        if let Ok((entity, transform, children)) = score_parent.get_single() {
            // 기존 제거
            for &entity in children {
                if let Some(ec) = commands.get_entity(entity) {
                    ec.despawn_recursive();
                }
            }

            let score_str = config.score.to_string();
            let mut x_offset = 0.;
            let offset = 13.;
            let vstr = score_str
                .split("")
                .filter(|&f| f != "")
                .map(|str| {
                    let e = commands.spawn(
                        (
                            Name::new("num"),
                            SpriteBundle {
                                texture: fb_assets.get_large_num(str),
                                transform: Transform {
                                    translation: Vec3::new(x_offset, 0., 0.),
                                    ..default()
                                },
                                ..default()
                            }
                        )
                    ).id();
                    x_offset += offset;
                    return e;
                })
                .collect::<Vec<_>>();
            commands.entity(entity).push_children(vstr.as_slice());

            let tr = transform.translation;
            let adjust_x = -1. * (x_offset - offset) / 2.;
            commands.entity(entity).insert(Transform {
                translation: Vec3::new(adjust_x, tr.y, tr.z),
                ..default()
            });
        }
    }
}

#[test]
fn test() {
    let score = 1;
    let binding = score.to_string();
    let score_str = binding.split("").collect::<Vec<_>>();
    println!("score: {:?}", score_str);
}