use crate::{
    constant::TARGET_FRAME_RATE,
    states::{Assets, MyStates},
};
use avian2d::prelude::*;
use bevy::{prelude::*, transform::commands};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_kira_audio::AudioPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_tweening::TweeningPlugin;

use super::{
    assets::AssetPlugin, ffi::FfiPlugin, game::GamePlugin, main_menu::MainMenuPlugin,
    mask::MaskPlugin, picking_callback::PickingCallbackPlugin, resize::ResizePlugin,
    tween_callback::TweenCallbackPlugin,
};

/// 공통 게임 설정
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // 외부 플러그인 설정
        app.add_plugins(bevy_framepace::FramepacePlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(TweeningPlugin)
            .add_plugins(AudioPlugin)
            .insert_resource(Time::new_with(Physics::fixed_hz(TARGET_FRAME_RATE)))
            .insert_resource(FramepaceSettings {
                limiter: Limiter::from_framerate(TARGET_FRAME_RATE),
            });

        // 내부 내부 설정
        app.add_plugins(MaskPlugin)
            .add_plugins(ResizePlugin)
            .add_plugins(AssetPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(TweenCallbackPlugin)
            .add_plugins(PickingCallbackPlugin)
            .add_plugins(FfiPlugin)
            .add_plugins(GamePlugin);

        // 2d 카메라 생성
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        });

        #[cfg(feature = "inspector")]
        {
            use super::inspector::InspectorPlugin;
            app.add_plugins(InspectorPlugin)
                .add_plugins(PhysicsDebugPlugin::default());
        }
    }
}
