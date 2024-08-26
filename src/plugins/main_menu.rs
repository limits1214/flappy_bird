use bevy::prelude::*;

use crate::{
    states::MyStates,
    systems::{self, bird::bird_animation, ground::ground_animation, main_menu::title_animation},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::MainMenu), systems::main_menu::main_enter)
            .add_systems(OnExit(MyStates::MainMenu), systems::main_menu::exit)
            .add_systems(
                Update,
                (ground_animation, bird_animation, title_animation)
                    .run_if(in_state(MyStates::MainMenu)),
            );
    }
}
