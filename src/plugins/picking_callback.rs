use bevy::prelude::*;

use crate::{
    events::picking_callback::{
        JumpPickingEvent, MainToGamePickingEvent, PausePickingEvent, ResultToMainPickingEvent,
    },
    states::{Game, MyStates},
    systems::picking_callback::{bird_jump, main_to_game, puase, result_to_main},
};

pub struct PickingCallbackPlugin;

impl Plugin for PickingCallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JumpPickingEvent>()
            .add_event::<MainToGamePickingEvent>()
            .add_event::<PausePickingEvent>()
            .add_event::<ResultToMainPickingEvent>()
            .add_systems(
                Update,
                bird_jump.run_if(
                    on_event::<JumpPickingEvent>().and_then(
                        in_state(MyStates::Game(Game::Guide))
                            .or_else(in_state(MyStates::Game(Game::Game))),
                    ),
                ),
            )
            .add_systems(
                Update,
                main_to_game.run_if(on_event::<MainToGamePickingEvent>()),
            )
            .add_systems(Update, puase.run_if(on_event::<PausePickingEvent>()))
            .add_systems(
                Update,
                result_to_main.run_if(on_event::<ResultToMainPickingEvent>()),
            );
    }
}
