use bevy::prelude::*;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone)]
pub enum MyStates {
    Assets(Assets),
    MainMenu,
    Game(Game),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Assets {
    Loading,
    Gen,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Game {
    Init,
    Guide,
    Game,
    GuidePause,
    GamePause,
    Result,
}

pub mod prelude {
    pub use crate::states::*;
}
