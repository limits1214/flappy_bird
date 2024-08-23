
use bevy::prelude::*;

#[derive(Component)]
pub struct Bird;

#[derive(Component, Debug)]
pub struct BirdAnimateTimer(pub Timer);

#[derive(Bundle)]
pub struct BirdBundle {
    name: Name,
    bird: Bird,
    animate_timer: BirdAnimateTimer,
}

impl Default for BirdBundle {
    fn default() -> Self {
        BirdBundle {
            name: Name::new("bird"),
            animate_timer: BirdAnimateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            bird: Bird,
        }
    }
}

type BirdComponentGen = (Name, Bird, SpriteBundle, TextureAtlas, BirdAnimateTimer);

pub fn gen_bird_component(texture: Handle<Image>, atlas: Handle<TextureAtlasLayout>) -> BirdComponentGen {
    let bird  = (
        Name::new("bird"),
        Bird,
        SpriteBundle {
            texture: texture,
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: atlas,
        },
        BirdAnimateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    );
    bird
}