use std::f32::consts::PI;
use std::time::Duration;

use crate::components::prelude::*;
use crate::events::picking_callback::JumpPickingEvent;
use crate::events::prelude::*;
use crate::prelude::PAUSE_BTN_DEPTH;
use crate::resources::prelude::ResizeScale;
use crate::states::{Game, MyStates};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_tweening::lens::TransformRotationLens;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween};

pub fn bird_colliding_check(
    mut commands: Commands,
    q_bird_colliders: Query<&CollidingEntities, With<Bird>>,
    q_ground: Query<Entity, With<GroundCollider>>,
    q_pipe_point: Query<Entity, (With<PipePoint>, Without<PointEarned>)>,
    q_pipe: Query<Entity, With<Pipe>>,
    mut ew_score_up: EventWriter<ScoreUpEvent>,
    mut ew_result: EventWriter<ResultEvent>,
) {
    for colliding_entities in &q_bird_colliders {
        for entitiy in colliding_entities.iter() {
            if q_pipe.get(*entitiy).is_ok() || q_ground.get(*entitiy).is_ok() {
                info!("???");
                // next_state.set(States::Game(Game::Result));
                ew_result.send(ResultEvent);
                return;
            }

            // 점수 획득
            if let Ok(entity) = q_pipe_point.get(*entitiy) {
                info!("점수획득");
                if let Some(mut ec) = commands.get_entity(entity) {
                    ec.insert(PointEarned);
                    ew_score_up.send(ScoreUpEvent);
                }
            }
        }
    }
}
