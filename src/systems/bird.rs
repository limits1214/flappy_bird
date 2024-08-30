use crate::components::prelude::*;
use crate::events::prelude::*;
use crate::prelude::FlappyBirdAssets;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
pub fn bird_colliding_check(
    mut commands: Commands,
    q_bird_colliders: Query<&CollidingEntities, (With<Bird>, Without<AdRespawn>)>,
    q_ground: Query<Entity, With<GroundCollider>>,
    q_pipe_point: Query<Entity, (With<PipePoint>, Without<PointEarned>)>,
    q_pipe: Query<Entity, With<Pipe>>,
    mut ew_score_up: EventWriter<ScoreUpEvent>,
    mut ew_result: EventWriter<ResultEvent>,
    audio: Res<Audio>,
    fb_assets: Res<FlappyBirdAssets>,
) {
    for colliding_entities in &q_bird_colliders {
        for entitiy in colliding_entities.iter() {
            if q_pipe.get(*entitiy).is_ok() || q_ground.get(*entitiy).is_ok() {
                audio.play(fb_assets.sfx_hit.clone());
                info!("???");
                // next_state.set(States::Game(Game::Result));
                ew_result.send(ResultEvent);
                return;
            }

            // 점수 획득
            if let Ok(entity) = q_pipe_point.get(*entitiy) {
                info!("점수획득");
                audio.play(fb_assets.sfx_point.clone());
                if let Some(mut ec) = commands.get_entity(entity) {
                    ec.insert(PointEarned);
                    ew_score_up.send(ScoreUpEvent);
                }
            }
        }
    }
}
