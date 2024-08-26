use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SparkleAniTimer(pub Timer);

#[derive(Component, Debug)]
pub struct ScoreCountingAniTimer(pub Timer);

#[derive(Component, Debug)]
pub struct BirdAnimateTimer(pub Timer);
