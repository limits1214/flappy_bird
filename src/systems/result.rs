use std::time::Duration;

use bevy::{color::palettes::css::{BLACK, WHITE}, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::{SpriteColorLens, TransformPositionLens}, Animator, BoxedTweenable, Delay, EaseFunction, Tracks, Tween, TweenCompleted, Tweenable};
use rand::Rng;

use crate::{components::{mask::MaskCenter, result::InResult, sparkle::{Sparkle, SparkleAniTimer}, Bg}, constant::{TWEEN_DEATH_WHITE, TWEEN_MASK_CENTER_BACK, TWEEN_RESULT_TO_MENU}, events::result::ResultEvent, ffi::{Ffi, FfiKv, Score}, resources::{assets::FlappyBirdAssets, config::GameConfig,  }, states::{Game, States}};

pub fn on_result(
    mut commands: Commands,
    mut reader: EventReader<ResultEvent>,
    q_bg: Query<Entity, With<Bg>>,
    fb_assets: Res<FlappyBirdAssets>,
    mut next_state: ResMut<NextState<States>>,
    mut q_mask_center: Query<(Entity, &mut Transform), With<MaskCenter>>,
) {
    for _ in reader.read() {
        info!("result");

        if let Ok((entity, mut transform)) = q_mask_center.get_single_mut() {
            transform.translation.z = 999.;
            // 화면 하약색 깜빡임
            let tween = Tween::new(
                EaseFunction::QuadraticInOut, 
                Duration::from_millis(300), 
                SpriteColorLens {
                    start: Color::srgba_u8(0, 0, 0, 0),
                    end: Color::WHITE,
                }
            );

            let tween2 = Tween::new(
                EaseFunction::QuadraticInOut, 
                Duration::from_millis(300), 
                SpriteColorLens {
                    start: Color::WHITE,
                    end: Color::srgba_u8(0, 0, 0, 0),
                }
            ).with_completed_event(TWEEN_DEATH_WHITE);
            let seq = tween.then(tween2);

            commands.entity(entity).insert(Animator::new(seq));
            next_state.set(States::Game(Game::Result));
        }


        // if let Ok(bg) = q_bg.get_single() {
        //     if let Some(mut ec) = commands.get_entity(bg) {

        //         let gameover = (
        //             Name::new("gameover"),
        //             InResult,
        //             SpriteBundle {
        //                 texture: fb_assets.label_game_over.clone(),
        //                 transform: Transform {
        //                     translation: Vec3::new(0., 50., 10.),
        //                     ..default()
        //                 },
        //                 ..default()
        //             }
        //         );

        //         let panel_parent = (
        //             Name::new("result parent"),
        //             InResult,
        //             SpatialBundle::default()
        //         );

        //         let panel = (
        //             Name::new("panel"),
        //             SpriteBundle {
        //                 texture: fb_assets.panel_score.clone(),
        //                 transform: Transform {
        //                     translation: Vec3::new(0., 0., 10.),
        //                     ..default()
        //                 },
        //                 ..default()
        //             }
        //         );

        //         let ok = (
        //             Name::new("ok"),
        //             InResult,
        //             SpriteBundle {
        //                 texture: fb_assets.button_ok.clone(),
        //                 transform: Transform {
        //                     translation: Vec3::new(0., -60., 10.),
        //                     ..default()
        //                 },
        //                 ..default()
        //             },
        //             On::<Pointer<Click>>::run(|mut next_state: ResMut<NextState<States>>| {
        //                 next_state.set(States::MainMenu);
        //             }),
        //         );

        //         let medal_parent = (
        //             Name::new("medal_parent"),
        //             SpatialBundle::from_transform(Transform{
        //                 translation: Vec3::new(-32., -4., 999.),
        //                 ..default()
        //             })
        //         );

        //         let medal = (
        //             Name::new("medal bronze"),
        //             SpriteBundle {
        //                 texture: fb_assets.medal_bronze.clone(),
        //                 ..default()
        //             }
        //         );

        //         let score_parent= (
        //             Name::new("score_parent"),
        //             SpatialBundle::from_transform(Transform{
        //                 translation: Vec3::new(37., 7., 999.),
        //                 ..default()
        //             })
        //         );

        //         let score = (

        //         );

        //         let best_parent = (
        //             Name::new("best_parent"),
        //             SpatialBundle::from_transform(Transform{
        //                 translation: Vec3::new(37., -14., 999.),
        //                 ..default()
        //             })
        //         );

        //         next_state.set(States::Game(Game::Result));
        //         ec.with_children(|parent| {
        //             parent.spawn(gameover);
        //             parent.spawn(
        //                 panel_parent
        //             )
        //             .with_children(|parent| {
        //                 parent.spawn(panel);
        //                 parent.spawn(medal_parent)
        //                     .with_children(|parent| {
        //                         parent.spawn(medal);
        //                     });
        //                 parent.spawn(score_parent)
        //                     .with_children(|parent| {
        //                         parent.spawn(
        //                             (
        //                                 SpriteBundle {
        //                                     texture: fb_assets.number_middle_1.clone(),
        //                                     ..default()
        //                                 }
        //                             )
        //                         );
        //                         parent.spawn(
        //                             (
        //                                 SpriteBundle {
        //                                     texture: fb_assets.number_middle_1.clone(),
        //                                     ..default()
        //                                 }
        //                             )
        //                         );
        //                     });
        //                 parent.spawn(best_parent)
        //                     .with_children(|parent| {
        //                         parent.spawn(
        //                             (
        //                                 SpriteBundle {
        //                                     texture: fb_assets.number_middle_1.clone(),
        //                                     ..default()
        //                                 }
        //                             )
        //                         );
        //                         parent.spawn(
        //                             (
        //                                 SpriteBundle {
        //                                     texture: fb_assets.number_middle_1.clone(),
        //                                     ..default()
        //                                 }
        //                             )
        //                         );
        //                     });
        //             });

        //             parent.spawn(ok);
        //         });
        //     }
        // }
    }
}

pub fn tween_callback_death_white(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
    mut q_mask: Query<&mut Transform, With<MaskCenter>>,
    q_bg: Query<Entity, With<Bg>>,
    fb_assets: Res<FlappyBirdAssets>,
    mut config: ResMut<GameConfig>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_DEATH_WHITE {
            if let Ok(mut transform) = q_mask.get_single_mut() {
                transform.translation.z = -1.;
            }

            

            let loaded_score_str = Ffi::get("score");
            let mut loaded_best_score = match serde_json::from_str::<Score>(&loaded_score_str) {
                Ok(s) => {
                    s.score
                },
                Err(_) => {
                    0
                }
            };

            let now_score = config.score;

            let is_new = if now_score > loaded_best_score {
                loaded_best_score = now_score;
                let score = Score {
                    score: now_score
                };
                let score_string = serde_json::to_string(&score).unwrap_or(String::new());
                Ffi::set("score", &score_string);
                true
            } else {
                false
            };


            let tween_gameover1 = Tween::new(
                EaseFunction::QuarticInOut, 
                Duration::from_millis(100), 
                TransformPositionLens {
                    start: Vec3::new(0., 55., 10.),
                    end: Vec3::new(0., 60., 10.),
                }
            );

            let tween_gameover2 = Tween::new(
                EaseFunction::QuarticInOut, 
                Duration::from_millis(400), 
                TransformPositionLens {
                    start: Vec3::new(0., 60., 10.),
                    end: Vec3::new(0., 50., 10.),
                }
            );

            let tween_gamover_seq = tween_gameover1.then(tween_gameover2);

            let tween_gameover_alpha = Tween::new(
                EaseFunction::QuinticOut, 
                Duration::from_millis(300), 
                SpriteColorLens {
                    start: Color::srgba_u8(0, 0, 0, 0),
                    end: WHITE.into()
                }
            );



            let gameover_parent = (
                Name::new("gameover parent"),
                
                SpatialBundle::from_transform(Transform {
                    translation: Vec3::new(0., 50., 10.),
                    ..default()
                }),
                Animator::new(tween_gamover_seq),
            );
            let gameover = (
                Name::new("gameover"),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba_u8(0, 0, 0, 0),
                        ..default()
                    },
                    texture: fb_assets.label_game_over.clone(),
                    ..default()
                },
                Animator::new(tween_gameover_alpha)
            );



            let panel_tween_delay = Delay::new(Duration::from_millis(500));
            let panel_tween = Tween::new(
                EaseFunction::QuadraticInOut, 
                Duration::from_millis(500), 
                TransformPositionLens {
                    start: Vec3::new(0., -300., 222.),
                    end:  Vec3::new(-0., 0., 222.),
                }
            );
            let seq = panel_tween_delay.then(panel_tween);

            let panel_parent = (
                Name::new("panel parent"),
                
                SpatialBundle::from_transform(Transform{
                    translation: Vec3::new(0., -300., 222.),
                    ..default()
                }),
                Animator::new(seq),
            );

            let panel = (
                Name::new("panel"),
                SpriteBundle {
                    texture: fb_assets.panel_score.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., 0., 0.),
                        ..default()
                    },
                    ..default()
                }
            );

            let delay_ok = Delay::new(Duration::from_millis(1500));

            let tween_ok = Tween::new(
                EaseFunction::QuadraticInOut, 
                Duration::from_millis(100), 
                SpriteColorLens {
                    start: Color::srgba_u8(0, 0, 0, 0),
                    end: WHITE.into(),
                }
            );

            let seq = delay_ok.then(tween_ok);

            let ok = (
                Name::new("ok"),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba_u8(0, 0, 0, 0),
                        ..default()
                    },
                    texture: fb_assets.button_ok.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -60., 10.),
                        ..default()
                    },
                    ..default()
                },
                On::<Pointer<Down>>::target_component_mut::<Transform>(|event, transform| {
                    transform.translation.y = -61.;
                }),
                On::<Pointer<Up>>::target_component_mut::<Transform>(|event, transform| {
                    transform.translation.y = -60.;
                }),
                On::<Pointer<DragEnd>>::target_component_mut::<Transform>(|event, transform| {
                    transform.translation.y = -60.;
                }),
                On::<Pointer<Click>>::run(|mut q_mask: Query<(Entity, &mut Transform), With<MaskCenter>>, mut commands: Commands| {
                    if let Ok((entity, mut transform)) = q_mask.get_single_mut() {
                        transform.translation.z = 999.;
                        let transition_tween = Tween::new(
                            EaseFunction::QuarticInOut, 
                            Duration::from_millis(500), 
                            SpriteColorLens {
                                start: Color::srgba_u8(0, 0, 0, 0),
                                end: BLACK.into(),
                            },
                        )
                        .with_completed_event(TWEEN_RESULT_TO_MENU);
                        let transition_tween2 = Tween::new(
                            EaseFunction::QuarticInOut, 
                            Duration::from_millis(500), 
                            SpriteColorLens {
                                start: BLACK.into(),
                                end: Color::srgba_u8(0, 0, 0, 0),
                            },
                        ).with_completed_event(TWEEN_MASK_CENTER_BACK);
                        
                        let seq = transition_tween.then(transition_tween2);
                        commands.entity(entity).insert(Animator::new(seq));
                    }
                }),
                Animator::new(seq),
            );

            let medal_parent = (
                Name::new("medal_parent"),
                SpatialBundle::from_transform(Transform{
                    translation: Vec3::new(-32., -4., 222.),
                    ..default()
                }),
            );

            let medal = (
                Name::new("medal"),
                if now_score >= 4 {
                    SpriteBundle {
                        texture: fb_assets.medal_platinum.clone(),
                        ..default()
                    }
                } else if now_score >= 3 {
                    SpriteBundle {
                        texture: fb_assets.medal_gold.clone(),
                        ..default()
                    }
                } else if now_score >= 2 {
                    SpriteBundle {
                        texture: fb_assets.medal_silver.clone(),
                        ..default()
                    }
                } else if now_score >= 1 {
                    SpriteBundle {
                        texture: fb_assets.medal_bronze.clone(),
                        ..default()
                    }
                } else {
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(0., 0., -999.),
                            ..default()
                        },
                        ..default()
                    }
                }
            );

            let score_parent= (
                Name::new("score_parent"),
                SpatialBundle::from_transform(Transform{
                    translation: Vec3::new(37., 7., 222.),
                    ..default()
                })
            );

            let score_str = now_score.to_string();
            let mut x_offset = 0.;
            let offset = 13.;
            let vstr_now = score_str
                .split("")
                .filter(|&f| f != "")
                .map(|str| {
                    let e = commands.spawn(
                        (
                            Name::new("num"),
                            SpriteBundle {
                                texture: fb_assets.get_middle_num(str),
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


            let best_parent = (
                Name::new("best_parent"),
                SpatialBundle::from_transform(Transform{
                    translation: Vec3::new(37., -14., 222.),
                    ..default()
                })
            );

            let score_str = loaded_best_score.to_string();
            let mut x_offset = 0.;
            let offset = 13.;
            let vstr_best = score_str
                .split("")
                .filter(|&f| f != "")
                .map(|str| {
                    let e = commands.spawn(
                        (
                            Name::new("num"),
                            SpriteBundle {
                                texture: fb_assets.get_middle_num(str),
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

            let result_parent = (
                Name::new("result parent"),
                InResult,
                SpatialBundle::default(),
            );

            let new = (
                Name::new("new"),
                SpriteBundle {
                    texture: fb_assets.label_new.clone(),
                    transform: Transform {
                        translation: Vec3::new(19., -4., 10.),
                        ..default()
                    },
                    ..default()
                }
            );

            let sparkle = (
                Name::new("spakle"),
                Sparkle,
                SparkleAniTimer(Timer::new(Duration::from_millis(150), TimerMode::Repeating)),
                SpriteBundle {
                    texture: fb_assets.gen_sparkle_atlas_texture.clone(),
                    ..default()
                },
                TextureAtlas {
                    index: 0,
                    layout: fb_assets.gen_sparkle_atlas_layout.clone(),
                }
            );

            let entity = q_bg.single();
            commands.entity(entity)
                .with_children(|parent| {
                    parent.spawn(result_parent)
                        .with_children(|parent| {
                            parent.spawn(gameover_parent)
                            .with_children(|parent| {
                                parent.spawn(gameover);
                            });
                            parent.spawn(panel_parent)
                                .with_children(|parent| {
                                    parent.spawn(panel);
                                    parent.spawn(score_parent)
                                        .push_children(vstr_now.as_slice());
                                    parent.spawn(best_parent)
                                        .push_children(vstr_best.as_slice());
                                    if is_new {
                                        parent.spawn(new);
                                    }
                                    if now_score >= 1 {
                                        parent.spawn(medal_parent)
                                        .with_children(|parent| {
                                            parent.spawn(medal);
                                            parent.spawn(sparkle);
                                        });
                                    }
                                });
                            parent.spawn(ok);
                        });
                });
        }
    }
}

pub fn tween_callback_result_to_menu(
    mut reader: EventReader<TweenCompleted>,
    mut next_state: ResMut<NextState<States>>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_RESULT_TO_MENU {
            next_state.set(States::MainMenu);
        }
    }
}

pub fn tween_callback_mask_center_back(
    mut reader: EventReader<TweenCompleted>,
    mut q_mask: Query<&mut Transform, With<MaskCenter>>
) {
    for event in reader.read() {
        if event.user_data == TWEEN_MASK_CENTER_BACK {
            if let Ok(mut transform) = q_mask.get_single_mut() {
                transform.translation.z = -1.;
            }
        }
    }
}

pub fn spakle_animation(
    atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut q_ani: Query<(&mut TextureAtlas, &mut SparkleAniTimer, &mut Transform), With<Sparkle>>
) {
    for (mut at, mut ti, mut transform) in &mut q_ani {
        ti.0.tick(time.delta());
        if ti.0.just_finished() {
            let a = &at.layout;
            let a = atlases.get(a.id()).unwrap();
            
            at.index = (at.index + 1) % a.textures.len();
            if at.index == 0 {
                let rx = rand::thread_rng().gen_range(-8.0..8.0);
                let ry = rand::thread_rng().gen_range(-8.0..8.0);
                transform.translation.x = rx;
                transform.translation.y = ry;
            }
        }
    }
}