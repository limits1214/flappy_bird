use std::f32::consts::PI;
use std::time::Duration;

use crate::components::game::{Bird, Guide};
use crate::events::game::JumpEvent;
use crate::states::{Game, MyStates};
use crate::{constant::PAUSE_BTN_DEPTH, resources::resize_scale::ResizeScale};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_tweening::lens::TransformRotationLens;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween};

pub fn touch(
    mut commands: Commands,
    mut read: EventReader<JumpEvent>,
    q_guide: Query<Entity, With<Guide>>,
    q_bird: Query<Entity, With<Bird>>,
    resize_scale: Res<ResizeScale>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    for JumpEvent(_, a) in read.read() {
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
