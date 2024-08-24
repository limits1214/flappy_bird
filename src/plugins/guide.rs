use bevy::prelude::*;

use crate::{events::jump::JumpEvent, states::{Game, States}, systems::{self, bird::{bird_animation}, ground::ground_animation, touch::touch}};

pub struct GuidePlugin;

impl Plugin for GuidePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<JumpEvent>()
            .add_systems(OnEnter(States::Game(Game::Guide)), systems::guide::enter)
            .add_systems(
                Update,
                (
                    bird_animation, ground_animation, touch
                ).run_if(
                    in_state(States::Game(Game::Guide))
                        .or_else(in_state(States::Game(Game::Game)))
                )
            );
    }
}