use std::time::Duration;
use crate::{components::{game::{BestScore, NowScore, Sparkle, }, timer::{ScoreCountingAniTimer, SparkleAniTimer}}, constant::{BRONZE_MEDAL_CUT, GOLD_MEDAL_CUT, PLATINUM_MEDAL_CUT, SILVER_MEDAL_CUT, TWEEN_SPARKLE_START}, events::game::ResultEvent, my_extensions::*};
use bevy::{color::palettes::css::{BLACK, WHITE}, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::{SpriteColorLens, TransformPositionLens}, Animator, BoxedTweenable, Delay, EaseFunction, Tracks, Tween, TweenCompleted, Tweenable};
use rand::Rng;

use crate::{components::{mask::MaskCenter}, constant::{TWEEN_DEATH_WHITE, TWEEN_MASK_CENTER_BACK, TWEEN_PANEL_UP_END, TWEEN_RESULT_TO_MENU}, ffi::{Ffi, FfiKv, Score}, resources::{assets::FlappyBirdAssets, game::GameConfig,  }, states::{Game, States}};

use super::score::{get_score_entitiy_vec, scoring_helper2, ScoreingHelperArgs};

pub fn on_result(
    mut commands: Commands,
    mut reader: EventReader<ResultEvent>,
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

pub fn score_couting_ani(
    mut commands: Commands,
    fb_assets: Res<FlappyBirdAssets>,
    time: Res<Time>,
    mut q_ani: Query<(Entity, &mut ScoreCountingAniTimer), With<ScoreCountingAniTimer>>,
    config: Res<GameConfig>,
    mut q_now_score: Query<(Entity, &mut NowScore, &Children, &Transform), With<NowScore>>,
    mut q_best_score: Query<(Entity, &mut BestScore, &Children, &Transform), With<BestScore>>,
    q_test: Query<(Entity, &Transform, &Children), With<NowScore>>,
) {
    for (timer_entity, mut timer) in &mut q_ani {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {

            
            let (entity, mut nowscore, children, transform) = q_now_score.single_mut();

            for &entity in children {
                if let Some(ec) = commands.get_entity(entity) {
                    ec.despawn_recursive();
                }
            }

            let score_str = nowscore.0.to_string();
            let mut x_offset = 0.;
            let offset = 8.;
            let vstr_now = score_str
                .split("")
                .filter(|&f| f != "")
                .map(|str| {
                    let e = commands.spawn(
                        (
                            Name::new("num?"),
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

            let adjust_x = -1. * (x_offset - 8.) / 2.;
            let tr = transform.translation;
            commands.entity(entity).insert(Transform {
                translation: Vec3::new(37. + adjust_x, tr.y, tr.z),
                ..default()
            });
            commands.entity(entity).push_children(vstr_now.as_slice());
            if nowscore.0 >= config.score {
                commands.entity(timer_entity).remove::<ScoreCountingAniTimer>();
            }
            

            nowscore.0 += 1;

            let (entity, mut best_score, children, transform) = q_best_score.single_mut();

            if nowscore.0 > best_score.0 {
                for &entity in children {
                    if let Some(ec) = commands.get_entity(entity) {
                        ec.despawn_recursive();
                    }
                }
    
                let score_str = best_score.0.to_string();
                let mut x_offset = 0.;
                let offset = 8.;
                let vstr_now = score_str
                    .split("")
                    .filter(|&f| f != "")
                    .map(|str| {
                        let e = commands.spawn(
                            (
                                Name::new("num?"),
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
    
                let adjust_x = -1. * (x_offset - 8.) / 2.;

                let tr = transform.translation;
                commands.entity(entity).insert(Transform {
                    translation: Vec3::new(37. + adjust_x, tr.y, tr.z),
                    ..default()
                });
                commands.entity(entity).push_children(vstr_now.as_slice());
                // if best_score.0 >= config.score {
                //     commands.entity(timer_entity).remove::<ScoreCountingAniTimer>();
                // }
    
                best_score.0 += 1;
            }
        }
    }
}
