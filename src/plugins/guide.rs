use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

use crate::{events::{jump::JumpEvent, result::ResultEvent, score::ScoreUpEvent}, resources::{config::GameConfig, pipe_spawn_timer::PipeSpawnTimer}, states::{Game, States}, systems::{self, bird::{bird_animation, bird_colliding_check}, game::trsition_result_on_main, ground::ground_animation, main_menu::{tween_callback_mask_center_back, tween_callback_menu_to_game}, pipe::{pipe_movement}, result::on_result, score::score_up, touch::touch}};

pub struct GuidePlugin;

impl Plugin for GuidePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<JumpEvent>()
            .add_event::<ScoreUpEvent>()
            .add_event::<ResultEvent>()
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(1., TimerMode::Repeating)))
            .insert_resource(GameConfig { score: 0 })
            .add_systems(OnEnter(States::Game(Game::Guide)), (systems::guide::enter))
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
                (score_up)
                    .after(bird_colliding_check)
                    .run_if(on_event::<ScoreUpEvent>())
            )
            .add_systems(
                Update, 
                (on_result)
                    .after(bird_colliding_check)
                    .run_if(on_event::<ResultEvent>())
            )
            .add_systems(
                OnTransition {
                    entered: States::MainMenu,
                    exited: States::Game(Game::Result),
                }, trsition_result_on_main
            )
            .add_systems(
                OnTransition {
                    entered: States::Game(Game::Game),
                    exited: States::Game(Game::Result),
                }, trsition_result_on_main
            )
            .add_systems(
                Update, 
                (
                    tween_callback_menu_to_game,
                    tween_callback_mask_center_back
                )
                    .run_if(on_event::<TweenCompleted>())
            )
            ;
    }
}