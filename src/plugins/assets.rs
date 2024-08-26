use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};

use crate::states::Assets::{Gen, Loading};
use crate::{
    resources::assets::FlappyBirdAssets,
    states::MyStates,
    systems::assets::{assets_gen, loading_ui},
};
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MyStates::Assets(Loading))
            .add_systems(OnEnter(MyStates::Assets(Loading)), loading_ui)
            .add_loading_state(
                LoadingState::new(MyStates::Assets(Loading))
                    .load_collection::<FlappyBirdAssets>()
                    .continue_to_state(MyStates::Assets(Gen)),
            )
            .add_systems(OnEnter(MyStates::Assets(Gen)), assets_gen);
    }
}
