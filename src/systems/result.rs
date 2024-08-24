use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{components::{result::InResult, Bg}, events::result::ResultEvent, resources::assets::FlappyBirdAssets, states::{Game, States}};

pub fn on_result(
    mut commands: Commands,
    mut reader: EventReader<ResultEvent>,
    q_bg: Query<Entity, With<Bg>>,
    fb_assets: Res<FlappyBirdAssets>,
    mut next_state: ResMut<NextState<States>>
) {
    for _ in reader.read() {
        info!("result");
        if let Ok(bg) = q_bg.get_single() {
            if let Some(mut ec) = commands.get_entity(bg) {

                let panel_parent = (
                    Name::new("result parent"),
                    InResult,
                    SpatialBundle::default()
                );

                let panel = (
                    Name::new("panel"),
                    SpriteBundle {
                        texture: fb_assets.panel_score.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 0., 10.),
                            ..default()
                        },
                        ..default()
                    }
                );

                let ok = (
                    Name::new("ok"),
                    SpriteBundle {
                        texture: fb_assets.button_ok.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., -60., 10.),
                            ..default()
                        },
                        ..default()
                    },
                    On::<Pointer<Click>>::run(|mut next_state: ResMut<NextState<States>>| {
                        next_state.set(States::MainMenu);
                    }),
                );

                let medal_parent = (
                    Name::new("medal_parent"),
                    SpatialBundle::from_transform(Transform{
                        translation: Vec3::new(-32., -4., 999.),
                        ..default()
                    })
                );

                let medal = (
                    Name::new("medal bronze"),
                    SpriteBundle {
                        texture: fb_assets.medal_bronze.clone(),
                        ..default()
                    }
                );

                let score_parent= (
                    Name::new("score_parent"),
                    SpatialBundle::from_transform(Transform{
                        translation: Vec3::new(37., 7., 999.),
                        ..default()
                    })
                );

                let score = (

                );

                let best_parent = (
                    Name::new("best_parent"),
                    SpatialBundle::from_transform(Transform{
                        translation: Vec3::new(37., -14., 999.),
                        ..default()
                    })
                );

                next_state.set(States::Game(Game::Result));
                ec.with_children(|parent| {
                    parent.spawn(
                        panel_parent
                    )
                    .with_children(|parent| {
                        parent.spawn(panel);
                        parent.spawn(medal_parent)
                            .with_children(|parent| {
                                parent.spawn(medal);
                            });
                        parent.spawn(score_parent)
                            .with_children(|parent| {
                                parent.spawn(
                                    (
                                        SpriteBundle {
                                            texture: fb_assets.number_middle_1.clone(),
                                            ..default()
                                        }
                                    )
                                );
                                parent.spawn(
                                    (
                                        SpriteBundle {
                                            texture: fb_assets.number_middle_1.clone(),
                                            ..default()
                                        }
                                    )
                                );
                            });
                        parent.spawn(best_parent)
                            .with_children(|parent| {
                                parent.spawn(
                                    (
                                        SpriteBundle {
                                            texture: fb_assets.number_middle_1.clone(),
                                            ..default()
                                        }
                                    )
                                );
                                parent.spawn(
                                    (
                                        SpriteBundle {
                                            texture: fb_assets.number_middle_1.clone(),
                                            ..default()
                                        }
                                    )
                                );
                            });
                        parent.spawn(ok);
                    });
                });
            }
        }
    }
}