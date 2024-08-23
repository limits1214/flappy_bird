use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

use crate::{constant::TARGET_FRAME_RATE, states::States};

use super::{assets::AssetPlugin, main_menu::MainMenuPlugin, mask::MaskPlugin, resize::ResizePlugin};

/// 공통 게임 설정
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // 외부 설정
        app
            .add_plugins(bevy_framepace::FramepacePlugin)
            .insert_resource(FramepaceSettings{
                limiter: Limiter::from_framerate(TARGET_FRAME_RATE)
            });

        // 내부 설정
        app
            .insert_state(States::AssetsLoading)
            .add_plugins(MaskPlugin)
            .add_plugins(ResizePlugin)
            .add_plugins(AssetPlugin)
            .add_plugins(MainMenuPlugin);

        // 2d 카메라 생성
        app
            .world_mut()
            .commands()
            .spawn(Camera2dBundle::default());

        #[cfg(feature="inspector")]
        {
            use super::inspector::InspectorPlugin;
            app
                .add_plugins(InspectorPlugin);
        }
    }
}