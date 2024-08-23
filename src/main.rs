use bevy::prelude::*;
use bevy_flappy_bird::AppPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
        )
        .add_plugins(AppPlugin)
        .run();
}
