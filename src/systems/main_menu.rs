use bevy::prelude::*;

use crate::resources::assets::FlappyBirdAssets;

pub fn enter(
    mut commands: Commands,
    fb_assets: Res<FlappyBirdAssets>
) {
    info!("main_menu_enter");
    let bg = (
        Name::new("bg"),
        SpriteBundle {
            texture: fb_assets.background_day.clone(),
            ..default()
        }
    );

    let bird = (
        Name::new("bird"),
        SpriteBundle {
            texture: fb_assets.gen_bird_atlas_texture.clone(),
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: fb_assets.gen_bird_atlas_layout.clone(),
        }
    );
    
    commands.spawn(
        bg
    )
    .with_children(|parent| {
        parent.spawn(
            bird
        );
    });
}

pub fn exit() {
    info!("main_menu_exit");
}