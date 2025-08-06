use bevy::prelude::*;
use bevy_flappy_bird::AppPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_app() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some(String::from("#target")),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AppPlugin)
        .run();
}
