use bevy::prelude::*;

use crate::{events::btn::PlayBtnClickEvent, states::States, systems::{self, bird::bird_animation, ground::ground_animation, main_menu::{play_btn_click, title_animation}}};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayBtnClickEvent>()
            .add_systems(OnEnter(States::MainMenu), systems::main_menu::enter)
            .add_systems(OnExit(States::MainMenu), systems::main_menu::exit)
            .add_systems(Update, (ground_animation, bird_animation, title_animation).run_if(in_state(States::MainMenu)))
            .add_systems(Update, play_btn_click.run_if(on_event::<PlayBtnClickEvent>()));
    }
}