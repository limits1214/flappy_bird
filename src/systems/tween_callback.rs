use crate::events::picking_callback::ResultToMainPickingEvent;
use crate::prelude::*;

pub fn tween_callback_menu_to_game(
    mut reader: EventReader<TweenCompleted>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_MENU_TO_GAME {
            next_state.set(MyStates::Game(Game::Init));
        }
    }
}

pub fn tween_callback_mask_center_back(
    mut reader: EventReader<TweenCompleted>,
    mut q_mask: Query<&mut Transform, With<MaskCenter>>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_MASK_CENTER_BACK {
            if let Ok(mut transform) = q_mask.get_single_mut() {
                transform.translation.z = -1.;
            }
        }
    }
}

pub fn tween_callback_spakle_start(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
    q_sparkle: Query<Entity, With<Sparkle>>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_SPARKLE_START {
            let sparkle = q_sparkle.single();
            commands
                .entity(sparkle)
                .insert(SparkleAniTimer(Timer::from_seconds(
                    0.1,
                    TimerMode::Repeating,
                )));
        }
    }
}

pub fn tween_callback_death_white(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
    mut q_mask: Query<&mut Transform, (With<MaskCenter>, Without<BestScore>)>,
    fb_assets: Res<FlappyBirdAssets>,
    q_bg: Query<Entity, With<Bg>>,
    mut config: ResMut<GameConfig>,
) {
    reader
        .read()
        .filter(|&event| event.user_data == TWEEN_DEATH_WHITE)
        .for_each(|_event| {
            if let Ok(mut transform) = q_mask.get_single_mut() {
                transform.translation.z = -1.;
            }

            let loaded_score_str = Ffi::get("score");
            let mut loaded_best_score = match serde_json::from_str::<Score>(&loaded_score_str) {
                Ok(s) => s.score,
                Err(_) => 0,
            };

            let result_parent = (
                Name::new("result parent"),
                InResult,
                SpatialBundle::default(),
            );

            let tween_gameover1 = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_millis(100),
                TransformPositionLens {
                    start: Vec3::new(0., 55., 10.),
                    end: Vec3::new(0., 60., 10.),
                },
            );

            let tween_gameover2 = Tween::new(
                EaseFunction::QuarticInOut,
                Duration::from_millis(400),
                TransformPositionLens {
                    start: Vec3::new(0., 60., 10.),
                    end: Vec3::new(0., 50., 10.),
                },
            );

            let tween_gamover_seq = tween_gameover1.then(tween_gameover2);

            let tween_gameover_alpha = Tween::new(
                EaseFunction::QuinticOut,
                Duration::from_millis(300),
                SpriteColorLens {
                    start: Color::srgba_u8(0, 0, 0, 0),
                    end: WHITE.into(),
                },
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
                Animator::new(tween_gameover_alpha),
            );

            let panel_tween_delay = Delay::new(Duration::from_millis(500));
            let panel_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(500),
                TransformPositionLens {
                    start: Vec3::new(0., -300., Z_INDEX_3),
                    end: Vec3::new(-0., 0., Z_INDEX_3),
                },
            )
            .with_completed_event(TWEEN_PANEL_UP_END);
            let seq = panel_tween_delay.then(panel_tween);
            let panel_parent = (
                Name::new("panel parent"),
                PanelParent,
                SpatialBundle::from_transform(Transform {
                    translation: Vec3::new(0., -300., Z_INDEX_3),
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
                },
            );

            let now_score_parent = (
                Name::new("now_score_parent"),
                NowScore(0),
                SpatialBundle::from_transform(Transform {
                    translation: Vec3::new(37., 7., Z_INDEX_3),
                    ..default()
                }),
            );

            let now_score_0 = (
                Name::new("num"),
                SpriteBundle {
                    texture: fb_assets.number_middle_0.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., 0., 0.),
                        ..default()
                    },
                    ..default()
                },
            );

            let score_str = loaded_best_score.to_string();
            let mut x_offset = 0.;
            let vstr_best = get_score_entitiy_vec(
                &mut commands,
                &fb_assets,
                false,
                8.,
                loaded_best_score,
                &mut x_offset,
            );
            // let offset = 8.;
            // let vstr_best = score_str
            //     .split("")
            //     .filter(|&f| f != "")
            //     .map(|str| {
            //         let e = commands.spawn(
            //             (
            //                 Name::new("num"),
            //                 SpriteBundle {
            //                     texture: fb_assets.get_middle_num(str),
            //                     transform: Transform {
            //                         translation: Vec3::new(x_offset, 0., 0.),
            //                         ..default()
            //                     },
            //                     ..default()
            //                 }
            //             )
            //         ).id();
            //         x_offset += offset;
            //         return e;
            //     })
            //     .collect::<Vec<_>>();
            let adjust_x = -1. * (x_offset - 8.) / 2.;
            let best_score_parent = (
                Name::new("best_score_parent"),
                BestScore(loaded_best_score),
                SpatialBundle::from_transform(Transform {
                    translation: Vec3::new(37. + adjust_x, -14., Z_INDEX_3),
                    ..default()
                }),
            );
            let entity = q_bg.single();
            commands.entity(entity).with_children(|parent| {
                parent.spawn(result_parent).with_children(|parent| {
                    parent.spawn(gameover_parent).with_children(|parent| {
                        parent.spawn(gameover);
                    });
                    parent.spawn(panel_parent).with_children(|parent| {
                        parent.spawn(panel).with_children(|parent| {
                            parent.spawn(now_score_parent).with_children(|parent| {
                                parent.spawn(now_score_0);
                            });
                            parent
                                .spawn(best_score_parent)
                                .push_children(vstr_best.as_slice());
                        });

                        // if is_new {
                        //     parent.spawn(new);
                        // }
                        // if now_score >= 1 {
                        //     parent.spawn(medal_parent)
                        //     .with_children(|parent| {
                        //         parent.spawn(medal);
                        //         parent.spawn(sparkle);
                        //     });
                        // }
                    });
                    // parent.spawn(ok);
                });
            });
        });
}

pub fn tween_callback_result_to_menu(
    mut reader: EventReader<TweenCompleted>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_RESULT_TO_MENU {
            next_state.set(MyStates::MainMenu);
        }
    }
}

pub fn tween_callback_panel_up(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
    fb_assets: Res<FlappyBirdAssets>,
    mut config: ResMut<GameConfig>,
    mut q_panel_parent: Query<Entity, With<PanelParent>>,
    mut q_now_score: Query<Entity, With<NowScore>>,
    mut q_best_score: Query<Entity, With<BestScore>>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_PANEL_UP_END {
            let loaded_score_str = Ffi::get("score");
            let mut loaded_best_score = match serde_json::from_str::<Score>(&loaded_score_str) {
                Ok(s) => s.score,
                Err(_) => 0,
            };

            let now_score = config.score;

            let is_new = if now_score > loaded_best_score {
                loaded_best_score = now_score;
                let score = Score { score: now_score };
                let score_string = serde_json::to_string(&score).unwrap_or(String::new());
                Ffi::set("score", &score_string);
                true
            } else {
                false
            };

            let target_mill = 1000;
            let tick_mill = target_mill / (now_score + 1) as u64;

            let delay_ok = Delay::new(Duration::from_millis(1000));

            let tween_ok = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(100),
                TransformPositionLens {
                    start: Vec3::new(0., -40., Z_INDEX_MINUS),
                    end: Vec3::new(0., -40., Z_INDEX_3),
                },
            );

            let ok_seq = delay_ok.then(tween_ok);

            let ok = (
                Name::new("ok"),
                SpriteBundle {
                    // sprite: Sprite {
                    //     color: Color::srgba_u8(0, 0, 0, 0),
                    //     ..default()
                    // },
                    texture: fb_assets.button_ok.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -40., Z_INDEX_MINUS),
                        ..default()
                    },
                    ..default()
                },
                On::<Pointer<Down>>::target_component_mut::<Transform>(|_, transform| {
                    transform.translation.y = -41.;
                }),
                On::<Pointer<Up>>::target_component_mut::<Transform>(|event, transform| {
                    transform.translation.y = -40.;
                }),
                On::<Pointer<DragEnd>>::target_component_mut::<Transform>(|event, transform| {
                    transform.translation.y = -40.;
                }),
                On::<Pointer<Click>>::send_event::<ResultToMainPickingEvent>(),
                Animator::new(ok_seq),
            );

            match fb_assets.button_ad.clone() {
                Handle::Strong(_) => {
                    info!("handle strong");
                }
                Handle::Weak(_) => {
                    info!("handle weak");
                }
            }
            let delay_ad = Delay::new(Duration::from_millis(1000));

            let tween_ad = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(100),
                TransformPositionLens {
                    start: Vec3::new(0., -60., Z_INDEX_MINUS),
                    end: Vec3::new(0., -60., Z_INDEX_3),
                },
            );

            let ad_seq = delay_ad.then(tween_ad);
            let ad = (
                Name::new("ad btn"),
                SpriteBundle {
                    texture: fb_assets.button_ad.clone(),
                    transform: Transform::from_xyz(0., -60., Z_INDEX_MINUS),
                    ..default()
                },
                Animator::new(ad_seq),
            );

            let medal_parent = (
                Name::new("medal_parent"),
                SpatialBundle::from_transform(Transform {
                    translation: Vec3::new(-32., -4., Z_INDEX_4),
                    ..default()
                }),
            );

            let medal_delay = Delay::new(Duration::from_millis(1000));
            let tween_medal = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(1),
                TransformPositionLens {
                    start: Vec3::new(0., 0., Z_INDEX_MINUS),
                    end: Vec3::new(0., 0., Z_INDEX_3),
                },
            );
            let medal_seq = medal_delay.then(tween_medal);

            let medal = (
                Name::new("medal"),
                Animator::new(medal_seq),
                if now_score >= PLATINUM_MEDAL_CUT.into() {
                    SpriteBundle {
                        texture: fb_assets.medal_platinum.clone(),
                        transform: Transform::from_xyz(0., 0., Z_INDEX_MINUS),
                        ..default()
                    }
                } else if now_score >= GOLD_MEDAL_CUT.into() {
                    SpriteBundle {
                        texture: fb_assets.medal_gold.clone(),
                        transform: Transform::from_xyz(0., 0., Z_INDEX_MINUS),
                        ..default()
                    }
                } else if now_score >= SILVER_MEDAL_CUT.into() {
                    SpriteBundle {
                        texture: fb_assets.medal_silver.clone(),
                        transform: Transform::from_xyz(0., 0., Z_INDEX_MINUS),
                        ..default()
                    }
                } else if now_score >= BRONZE_MEDAL_CUT.into() {
                    SpriteBundle {
                        texture: fb_assets.medal_bronze.clone(),
                        transform: Transform::from_xyz(0., 0., Z_INDEX_MINUS),
                        ..default()
                    }
                } else {
                    SpriteBundle {
                        sprite: Sprite::alpah_0_sprite(),
                        transform: Transform::from_xyz(0., 0., Z_INDEX_MINUS),
                        ..default()
                    }
                },
            );

            // let score_str = now_score.to_string();
            // let mut x_offset = 0.;
            // let offset = 13.;
            // let vstr_now = score_str
            //     .split("")
            //     .filter(|&f| f != "")
            //     .map(|str| {
            //         let e = commands.spawn(
            //             (
            //                 Name::new("num"),
            //                 SpriteBundle {
            //                     texture: fb_assets.get_middle_num(str),
            //                     transform: Transform {
            //                         translation: Vec3::new(x_offset, 0., 0.),
            //                         ..default()
            //                     },
            //                     ..default()
            //                 }
            //             )
            //         ).id();
            //         x_offset += offset;
            //         return e;
            //     })
            //     .collect::<Vec<_>>();

            let new_delay = Delay::new(Duration::from_millis(1000));
            let tween_new = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(100),
                SpriteColorLens {
                    start: Color::new_alpha_0(),
                    end: WHITE.into(),
                },
            );
            let new_seq = new_delay.then(tween_new);

            let new = (
                Name::new("new"),
                Animator::new(new_seq),
                SpriteBundle {
                    sprite: Sprite::alpah_0_sprite(),
                    texture: fb_assets.label_new.clone(),
                    transform: Transform {
                        translation: Vec3::new(19., -4., Z_INDEX_4),
                        ..default()
                    },
                    ..default()
                },
            );

            let sparkle_delay = Delay::new(Duration::from_millis(1001));
            let tween_sparkle = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(100),
                SpriteColorLens {
                    start: Color::new_alpha_0(),
                    end: WHITE.into(),
                },
            )
            .with_completed_event(TWEEN_SPARKLE_START);
            let sparkle_seq = sparkle_delay.then(tween_sparkle);

            let sparkle = (
                Name::new("spakle"),
                Sparkle,
                SpriteBundle {
                    sprite: Sprite::alpah_0_sprite(),
                    texture: fb_assets.gen_sparkle_atlas_texture.clone(),
                    transform: Transform::from_xyz(0., 0., Z_INDEX_5),
                    ..default()
                },
                TextureAtlas {
                    index: 0,
                    layout: fb_assets.gen_sparkle_atlas_layout.clone(),
                },
                Animator::new(sparkle_seq),
            );

            let pp_entity = q_panel_parent.single();
            commands.entity(pp_entity).with_children(|parent| {
                parent.spawn(ScoreCountingAniTimer(Timer::new(
                    Duration::from_millis(tick_mill),
                    TimerMode::Repeating,
                )));
                parent.spawn(ok);
                parent.spawn(ad);

                if is_new {
                    parent.spawn(new);
                }
                if now_score >= BRONZE_MEDAL_CUT.into() {
                    parent.spawn(medal_parent).with_children(|parent| {
                        parent.spawn(medal);
                        parent.spawn(sparkle);
                    });
                }
            });
        }
    }
}
