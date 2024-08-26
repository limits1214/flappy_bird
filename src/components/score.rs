use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreParent;

#[derive(Component, Debug)]
pub struct NowScore(pub u32);

#[derive(Component, Debug)]
pub struct BestScore(pub u32);