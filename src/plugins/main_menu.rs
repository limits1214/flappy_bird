use bevy::prelude::*;

use crate::{
    prelude::{
        animation::bird_animation,
        movement::{ground_movement, title_movement},
    },
    states::MyStates,
    systems::{self},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MyStates::MainMenu),
            systems::on_main_menu::main_enter,
        )
        .add_systems(OnExit(MyStates::MainMenu), systems::on_main_menu::exit)
        .add_systems(
            Update,
            (ground_movement, bird_animation, title_movement).run_if(in_state(MyStates::MainMenu)),
        );
    }
}
