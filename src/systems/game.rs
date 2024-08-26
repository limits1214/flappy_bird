use bevy::prelude::*;

use crate::{components::states::{InGame, InResult}, resources::game::GameConfig};

pub fn trsition_result_on_main(
    mut commands: Commands,
    q_in_game: Query<Entity, With<InGame>>,
    q_in_result: Query<Entity, With<InResult>>,
    mut config: ResMut<GameConfig>,
) {
    config.score = 0;
    for e in &q_in_game {
        if let Some(ec) = commands.get_entity(e) {
            ec.despawn_recursive();
        }
    }
}

pub fn trsition_result_to_game() {

}