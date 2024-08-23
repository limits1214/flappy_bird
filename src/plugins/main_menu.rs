use bevy::prelude::*;

use crate::{states::States, systems::{self, bird::bird_animation, ground::ground_animation, main_menu::title_animation}};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(States::MainMenu), systems::main_menu::enter)
            .add_systems(OnExit(States::MainMenu), systems::main_menu::exit)
            .add_systems(Update, (ground_animation, bird_animation, title_animation).run_if(in_state(States::MainMenu)));
    }
}