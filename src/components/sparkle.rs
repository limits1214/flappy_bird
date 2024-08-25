use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SparkleAniTimer(pub Timer);

#[derive(Component, Debug)]
pub struct SpakleSpawnTimer(pub Time);

#[derive(Component)]
pub struct Sparkle;