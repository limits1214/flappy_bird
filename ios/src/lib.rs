use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::{window::WindowMode, winit::WinitSettings};
use bevy_flappy_bird::AppPlugin;
#[bevy_main]
fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                // .set(RenderPlugin {
                //     render_creation: RenderCreation::Automatic(WgpuSettings {
                //         backends: Some(Backends::VULKAN),
                //         ..default()
                //     }),
                //     ..default()
                // })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        // present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .insert_resource(WinitSettings {
        //     focused_mode: bevy::winit::UpdateMode::reactive(Duration::from_millis(17)),
        //     unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(17)),
        // })
        .add_plugins(AppPlugin)
        .run();
}
