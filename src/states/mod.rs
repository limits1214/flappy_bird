use bevy::prelude::*;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone)]
pub enum States {
    AssetsLoading,
    AssetsGen,
    MainMenu,
    Guide,
    Game,
    Pause,
    Result,
}