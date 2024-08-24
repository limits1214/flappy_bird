use avian2d::prelude::{Collider, CollidingEntities};
use bevy::prelude::*;

use crate::{components::{bird::{Bird, BirdAnimateTimer}, ground::GroundCollider, pipe::{Pipe, PipePoint, PointEarned}}, events::{result::ResultEvent, score::ScoreUpEvent}, states::{Game, States}};

pub fn bird_animation(
    atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut q_ani: Query<(&mut TextureAtlas, &mut BirdAnimateTimer), With<Bird>>
) {
    for (mut at, mut ti) in &mut q_ani {
        ti.0.tick(time.delta());
        if ti.0.just_finished() {
            let a = &at.layout;
            let a = atlases.get(a.id()).unwrap();
            
            at.index = (at.index + 1) % a.textures.len();
        }
    }
}

pub fn bird_colliding_check(
    mut commands: Commands,
    q_bird_colliders: Query<&CollidingEntities, With<Bird>>,
    q_ground: Query<Entity, With<GroundCollider>>,
    q_pipe_point: Query<Entity, (With<PipePoint>, Without<PointEarned>)>,
    q_pipe: Query<Entity, With<Pipe>>,
    mut ew_score_up: EventWriter<ScoreUpEvent>,
    mut ew_result: EventWriter<ResultEvent>,
    mut next_state: ResMut<NextState<States>>,
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