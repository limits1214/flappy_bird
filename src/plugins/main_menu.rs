use bevy::prelude::*;

use crate::{states::States, systems};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(States::MainMenu), systems::main_menu::enter)
            .add_systems(OnExit(States::MainMenu), systems::main_menu::exit);
    }
}