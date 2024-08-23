use bevy::prelude::*;
use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};

use crate::{resources::assets::FlappyBirdAssets, states::States, systems::assets::{assets_gen, loading_ui}};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(States::AssetsLoading), loading_ui)
            .add_loading_state(
                LoadingState::new(States::AssetsLoading)
                    .continue_to_state(States::AssetsGen)
                    .load_collection::<FlappyBirdAssets>()
            )
            .add_systems(OnEnter(States::AssetsGen), assets_gen);
    }
}