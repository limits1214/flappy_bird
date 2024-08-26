use bevy::prelude::*;
use crate::events::game::{JumpEvent, ResultEvent, ScoreUpEvent};
use crate::resources::game::GameConfig;
use crate::states::{Game, States};
use crate::systems::game::trsition_result_on_main;
use crate::systems::pipe::pipe_movement;
use crate::systems::result::{on_result, score_couting_ani, spakle_animation};
use crate::systems::score::score_up;
use crate::systems::{self, touch};
use crate::systems::bird::{bird_animation, bird_colliding_check};
use crate::systems::ground::ground_animation;
use crate::plugins::game::touch::touch;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<JumpEvent>()
            .add_event::<ScoreUpEvent>()
            .add_event::<ResultEvent>()
            .insert_resource(GameConfig { score: 0 })
            .add_systems(OnEnter(States::Game(Game::Init)), (systems::guide::enter))
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
                Update, 
                (spakle_animation, score_couting_ani)
                    .run_if(in_state(States::Game(Game::Result)))
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
            ;
    }
}