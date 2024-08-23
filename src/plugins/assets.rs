use bevy::prelude::*;
use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

use crate::{resources::assets::FlappyBirdAssets, states::States, systems::assets::{assets_gen, loading_ui}};
use crate::states::Assets::{Loading, Gen};
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(States::Assets(Loading))
            .add_systems(OnEnter(States::Assets(Loading)), loading_ui)
            .add_loading_state(
                LoadingState::new(States::Assets(Loading))
                    .load_collection::<FlappyBirdAssets>()
                    .continue_to_state(States::Assets(Gen))
            )
            .add_systems(OnEnter(States::Assets(Gen)), assets_gen);
    }
}