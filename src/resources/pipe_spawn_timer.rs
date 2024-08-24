use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PipeSpawnTimer(pub Timer);