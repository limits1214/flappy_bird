use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_mod_picking::DefaultPickingPlugins;
use avian2d::prelude::*;
use crate::{constant::TARGET_FRAME_RATE, states::{Assets, States}};

use super::{assets::AssetPlugin, guide::GuidePlugin, main_menu::MainMenuPlugin, mask::MaskPlugin, resize::ResizePlugin};

/// 공통 게임 설정
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // 외부 플러그인 설정
        app
            .add_plugins(bevy_framepace::FramepacePlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(PhysicsPlugins::default())
            .insert_resource(Time::new_with(Physics::fixed_hz(TARGET_FRAME_RATE)))
            .insert_resource(FramepaceSettings{
                limiter: Limiter::from_framerate(TARGET_FRAME_RATE)
            });

        // 내부 내부 설정
        app
            .add_plugins(MaskPlugin)
            .add_plugins(ResizePlugin)
            .add_plugins(AssetPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(GuidePlugin);

        // 2d 카메라 생성
        app
            .world_mut()
            .commands()
            .spawn(Camera2dBundle::default());

        #[cfg(feature="inspector")]
        {
            use super::inspector::InspectorPlugin;
            app
                .add_plugins(InspectorPlugin)
                .add_plugins(PhysicsDebugPlugin::default());
        }
    }
}