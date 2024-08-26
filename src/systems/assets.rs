use bevy::prelude::*;

use crate::{prelude::FlappyBirdAssets, states::MyStates};

pub fn loading_ui() {
    info!("loading_ui");
}

pub fn assets_gen(
    mut fb_assets: ResMut<FlappyBirdAssets>,
    mut next_state: ResMut<NextState<MyStates>>,
    mut textures: ResMut<Assets<Image>>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("assets_gen");
    let bird_0 = fb_assets.bird_orange_0.id();
    let bird_1 = fb_assets.bird_orange_1.id();
    let bird_2 = fb_assets.bird_orange_2.id();
    let mut builder = TextureAtlasBuilder::default();
    builder.add_texture(Some(bird_0), textures.get(bird_0).unwrap());
    builder.add_texture(Some(bird_1), textures.get(bird_1).unwrap());
    builder.add_texture(Some(bird_2), textures.get(bird_2).unwrap());
    let (layout, texture) = builder.build().unwrap();
    fb_assets.gen_bird_atlas_layout = atlases.add(layout);
    fb_assets.gen_bird_atlas_texture = textures.add(texture);

    let sparkle_0 = fb_assets.spark_0.id();
    let sparkle_1 = fb_assets.spark_1.id();
    let sparkle_2 = fb_assets.spark_2.id();
    let mut builder = TextureAtlasBuilder::default();
    builder.add_texture(Some(sparkle_0), textures.get(sparkle_0).unwrap());
    builder.add_texture(Some(sparkle_1), textures.get(sparkle_1).unwrap());
    builder.add_texture(Some(sparkle_2), textures.get(sparkle_2).unwrap());
    let (layout, texture) = builder.build().unwrap();
    fb_assets.gen_sparkle_atlas_layout = atlases.add(layout);
    fb_assets.gen_sparkle_atlas_texture = textures.add(texture);

    next_state.set(MyStates::MainMenu);
}
