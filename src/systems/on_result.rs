use crate::{events::game::ResultEvent, prelude::MASK_CENTER_FORE_Z_INDEX, states::MyStates};
use bevy::prelude::*;
use bevy_tweening::{lens::SpriteColorLens, Animator, EaseFunction, Tween};
use std::time::Duration;

use crate::{components::mask::MaskCenter, constant::TWEEN_DEATH_WHITE, states::Game};
pub fn on_result(
    mut commands: Commands,
    mut reader: EventReader<ResultEvent>,
    mut next_state: ResMut<NextState<MyStates>>,
    mut q_mask_center: Query<(Entity, &mut Transform), With<MaskCenter>>,
) {
    for _ in reader.read() {
        info!("result");

        if let Ok((entity, mut transform)) = q_mask_center.get_single_mut() {
            transform.translation.z = MASK_CENTER_FORE_Z_INDEX;
            // 화면 하약색 깜빡임
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                SpriteColorLens {
                    start: Color::srgba_u8(0, 0, 0, 0),
                    end: Color::WHITE,
                },
            );

            let tween2 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                SpriteColorLens {
                    start: Color::WHITE,
                    end: Color::srgba_u8(0, 0, 0, 0),
                },
            )
            .with_completed_event(TWEEN_DEATH_WHITE);
            let seq = tween.then(tween2);

            commands.entity(entity).insert(Animator::new(seq));
            next_state.set(MyStates::Game(Game::Result));
        }
    }
}
