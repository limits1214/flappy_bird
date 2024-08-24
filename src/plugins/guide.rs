use bevy::prelude::*;

use crate::{events::{jump::JumpEvent, score::ScoreUpEvent}, resources::{config::GameConfig, pipe_spawn_timer::PipeSpawnTimer}, states::{Game, States}, systems::{self, bird::{bird_animation, bird_colliding_check}, ground::ground_animation, pipe::{pipe_movement, pipe_spawn}, score::score_up, touch::touch}};

pub struct GuidePlugin;

impl Plugin for GuidePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<JumpEvent>()
            .add_event::<ScoreUpEvent>()
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(1., TimerMode::Repeating)))
            .insert_resource(GameConfig { score: 0 })
            .add_systems(OnEnter(States::Game(Game::Guide)), (systems::guide::enter, pipe_spawn))
            .add_systems(
                Update,
                (
                    bird_animation, ground_animation, touch
                ).run_if(
                    in_state(States::Game(Game::Guide))
                        .or_else(in_state(States::Game(Game::Game)))
                )
            )
            .add_systems(
                Update, 
                (pipe_movement, bird_colliding_check)
                    .run_if(in_state(States::Game(Game::Game)))
            )
            .add_systems(
                Update,
                score_up
                    .run_if(on_event::<ScoreUpEvent>())
            )
            ;
    }
}