use avian2d::prelude::*;
use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{
    lens::{SpriteColorLens, TransformRotationLens},
    Animator, AnimatorState, Delay, EaseFunction, Tween,
};
use std::{f32::consts::PI, time::Duration};

use crate::{
    events::picking_callback::{JumpPickingEvent, PausePickingEvent, ResultToMainPickingEvent},
    prelude::{
        Bird, FlappyBirdAssets, Guide, MaskCenter, MASK_CENTER_FORE_Z_INDEX, PAUSE_BTN_DEPTH,
        TWEEN_MASK_CENTER_BACK, TWEEN_MENU_TO_GAME, TWEEN_RESULT_TO_MENU,
    },
    resources::prelude::ResizeScale,
    states::{Game, MyStates},
};

pub fn bird_jump(
    mut commands: Commands,
    mut read: EventReader<JumpPickingEvent>,
    q_guide: Query<Entity, With<Guide>>,
    q_bird: Query<Entity, With<Bird>>,
    resize_scale: Res<ResizeScale>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    for JumpPickingEvent(_, a) in read.read() {
        if *a == PAUSE_BTN_DEPTH {
            return;
        }
        info!("jump!");

        // 만약 가이드가 존재하면 가이드를 꺼준다.
        if let Ok(entity) = q_guide.get_single() {
            if let Some(ec) = commands.get_entity(entity) {
                ec.despawn_recursive();
            }

            if let Ok(entity) = q_bird.get_single() {
                if let Some(mut ec) = commands.get_entity(entity) {
                    ec.insert(RigidBody::Dynamic);
                }
            }
            next_state.set(MyStates::Game(Game::Game));
        }

        if let Ok(entity) = q_bird.get_single() {
            if let Some(mut ec) = commands.get_entity(entity) {
                // impulse
                let mut impulse = ExternalImpulse::default();
                let scale = resize_scale.0;
                let y = 1000. * scale;
                info!("{y}");
                impulse.apply_impulse(Vec2::new(0., y)); // * tr.scale.x
                ec.insert(impulse);
                ec.insert(LinearVelocity::ZERO);

                // tweening
                let tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(200),
                    TransformRotationLens {
                        start: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                        end: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 4.),
                    },
                );

                let delay = Delay::new(Duration::from_millis(100));

                let tween2 = Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(300),
                    TransformRotationLens {
                        start: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 4.),
                        end: Quat::from_euler(EulerRot::XYZ, 0., 0., -PI / 2.),
                    },
                );

                let seq = tween.then(delay).then(tween2);
                ec.insert(Animator::new(seq));
            }
        }
    }
}

pub fn main_to_game(
    mut q_mask: Query<(Entity, &mut Transform), With<MaskCenter>>,
    mut commands: Commands,
    audio: Res<Audio>,
    fb_assets: Res<FlappyBirdAssets>,
) {
    audio.play(fb_assets.sfx_swooshing.clone());
    if let Ok((entity, mut transform)) = q_mask.get_single_mut() {
        transform.translation.z = MASK_CENTER_FORE_Z_INDEX;
        let transition_tween = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_millis(500),
            SpriteColorLens {
                start: Color::srgba_u8(0, 0, 0, 0),
                end: BLACK.into(),
            },
        )
        .with_completed_event(TWEEN_MENU_TO_GAME);
        let transition_tween2 = Tween::new(
            EaseFunction::QuarticInOut,
            Duration::from_millis(500),
            SpriteColorLens {
                start: BLACK.into(),
                end: Color::srgba_u8(0, 0, 0, 0),
            },
        )
        .with_completed_event(TWEEN_MASK_CENTER_BACK);

        let seq = transition_tween.then(transition_tween2);
        commands.entity(entity).insert(Animator::new(seq));
    }
}

pub fn puase(
    mut read: EventReader<PausePickingEvent>,
    now_state: Res<State<MyStates>>,
    mut next_state: ResMut<NextState<MyStates>>,
    mut commands: Commands,
    mut time: ResMut<Time<Physics>>,
    fb_assets: Res<FlappyBirdAssets>,
    mut q_bird_tween: Query<&mut Animator<Transform>, With<Bird>>,
) {
    for PausePickingEvent(target, _) in read.read() {
        info!("?? {:?}", *now_state.get());
        let pause_sprite = fb_assets.button_pause.clone();
        let resume_sprite = fb_assets.button_resume.clone();
        match *now_state.get() {
            MyStates::Game(Game::GamePause) => {
                next_state.set(MyStates::Game(Game::Game));
                if let Some(mut ec) = commands.get_entity(*target) {
                    ec.insert(pause_sprite.clone());
                }
                time.unpause();
                if let Ok(mut ani) = q_bird_tween.get_single_mut() {
                    ani.state = AnimatorState::Playing;
                }
            }
            MyStates::Game(Game::GuidePause) => {
                next_state.set(MyStates::Game(Game::Guide));
                if let Some(mut ec) = commands.get_entity(*target) {
                    ec.insert(pause_sprite.clone());
                }
                time.unpause();
                if let Ok(mut ani) = q_bird_tween.get_single_mut() {
                    ani.state = AnimatorState::Playing;
                }
            }
            MyStates::Game(Game::Guide) => {
                next_state.set(MyStates::Game(Game::GuidePause));
                if let Some(mut ec) = commands.get_entity(*target) {
                    ec.insert(pause_sprite.clone());
                }
                time.pause();
                if let Ok(mut ani) = q_bird_tween.get_single_mut() {
                    ani.state = AnimatorState::Paused;
                }
            }
            MyStates::Game(Game::Game) => {
                next_state.set(MyStates::Game(Game::GamePause));
                if let Some(mut ec) = commands.get_entity(*target) {
                    ec.insert(pause_sprite.clone());
                }
                time.pause();
                if let Ok(mut ani) = q_bird_tween.get_single_mut() {
                    ani.state = AnimatorState::Paused;
                }
            }
            _ => {
                info!("zz");
            }
        };
    }
}

pub fn result_to_main(
    mut read: EventReader<ResultToMainPickingEvent>,
    mut q_mask: Query<(Entity, &mut Transform), With<MaskCenter>>,
    mut commands: Commands,
) {
    for _event in read.read() {
        if let Ok((entity, mut transform)) = q_mask.get_single_mut() {
            transform.translation.z = MASK_CENTER_FORE_Z_INDEX;
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
            )
            .with_completed_event(TWEEN_MASK_CENTER_BACK);

            let seq = transition_tween.then(transition_tween2);
            commands.entity(entity).insert(Animator::new(seq));
        }
    }
}
