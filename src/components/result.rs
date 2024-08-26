use bevy::prelude::*;

#[derive(Component)]
pub struct InResult;

#[derive(Component, Debug)]
pub struct ScoreCountingAniTimer(pub Timer);

#[derive(Component)]
pub struct PanelParent;