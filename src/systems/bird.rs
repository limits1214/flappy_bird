use bevy::prelude::*;

use crate::components::bird::{Bird, BirdAnimateTimer};

pub fn bird_animation(
    atlases: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut q_ani: Query<(&mut TextureAtlas, &mut BirdAnimateTimer), With<Bird>>
) {
    for (mut at, mut ti) in &mut q_ani {
        
        ti.0.tick(time.delta());
        if ti.0.just_finished() {
            let a = &at.layout;
            let a = atlases.get(a.id()).unwrap();
            
            at.index = (at.index + 1) % a.textures.len();
        }
    }
}