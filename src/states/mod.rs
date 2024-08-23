use bevy::prelude::*;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone)]
pub enum States {
    Assets(Assets),
    //
    MainMenu,
    //
    Guide,
    Game,
    Pause,
    Result,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Assets {
    Loading, Gen,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Game {
    Guide,
    Game,
    Pause,
    Result,
}