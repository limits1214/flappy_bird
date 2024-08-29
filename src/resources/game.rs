use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GameConfig {
    pub score: u32,
    pub is_ad_show: bool,
}
