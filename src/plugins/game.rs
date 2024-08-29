use crate::events::game::{ResultEvent, ScoreUpEvent};
use crate::prelude::animation::{
    bird_ad_spawn_ani, bird_animation, score_couting_ani, spakle_animation,
};
use crate::prelude::movement::{ground_movement, pipe_movement};
use crate::resources::game::GameConfig;
use crate::states::{Game, MyStates};
use crate::systems::prelude::*;
use bevy::prelude::*;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreUpEvent>()
            .add_event::<ResultEvent>()
            .insert_resource(GameConfig { score: 0 })
            .add_systems(OnEnter(MyStates::Game(Game::Init)), game_enter)
            .add_systems(
                Update,
                (bird_animation, ground_movement).run_if(
                    in_state(MyStates::Game(Game::Guide))
                        .or_else(in_state(MyStates::Game(Game::Game))),
                ),
            )
            .add_systems(
                Update,
                (pipe_movement, bird_colliding_check, bird_ad_spawn_ani)
                    .run_if(in_state(MyStates::Game(Game::Game))),
            )
            .add_systems(
                Update,
                (score_up)
                    .after(bird_colliding_check)
                    .run_if(on_event::<ScoreUpEvent>()),
            )
            .add_systems(
                Update,
                (on_result)
                    .after(bird_colliding_check)
                    .run_if(on_event::<ResultEvent>()),
            )
            .add_systems(
                Update,
                (spakle_animation, score_couting_ani)
                    .run_if(in_state(MyStates::Game(Game::Result))),
            )
            .add_systems(
                OnTransition {
                    entered: MyStates::MainMenu,
                    exited: MyStates::Game(Game::Result),
                },
                trsition_result_on_main,
            )
            .add_systems(
                OnTransition {
                    entered: MyStates::Game(Game::Game),
                    exited: MyStates::Game(Game::Result),
                },
                trsition_result_to_game,
            );
    }
}
