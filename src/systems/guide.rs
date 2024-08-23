use bevy::{prelude::*, transform::commands};
use bevy_mod_picking::prelude::*;
use crate::{components::{bird::BirdBundle, ground::Ground, guide::Guide, resize::Resizable, states::InGame}, constant::Z_INDEX_1, events::resize::ResizeEvent, resources::assets::FlappyBirdAssets, states::Game};
use crate::states::States;
pub fn enter(
    mut commands: Commands,
    fb_assets: Res<FlappyBirdAssets>,
    mut ew_resize: EventWriter<ResizeEvent>,
) {
    info!("guide_enter");
    let bg = (
        Name::new("bg"),
        InGame,
        Resizable,
        SpriteBundle {
            texture: fb_assets.background_day.clone(),
            ..default()
        }
    );

    let bird = (
        BirdBundle::default(),
        SpriteBundle {
            texture: fb_assets.gen_bird_atlas_texture.clone(),
            transform: Transform {
                translation: Vec3::new(-30., 30., Z_INDEX_1),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: fb_assets.gen_bird_atlas_layout.clone(),
        },
    );

    let pause_sprite = fb_assets.button_pause.clone();
    let resume_sprite = fb_assets.button_resume.clone();

    let pause_btn = (
        Name::new("pause"),
        SpatialBundle::from_transform(Transform {
            translation: Vec3::new(-55., 110., Z_INDEX_1),
            ..default()
        }),
        On::<Pointer<Click>>::run(move |
            event: Listener<Pointer<Click>>,
            now_state: Res<State<States>>,
            mut next_state: ResMut<NextState<States>>,
            mut commands: Commands
            | {
            info!("pause!!");
            match *now_state.get() {
                States::Game(Game::Pause) => {
                    next_state.set(States::Game(Game::Game));
                    if let Some(mut ec) = commands.get_entity(event.target) {
                        ec.insert(pause_sprite.clone());
                    }
                },
                _ => {
                    next_state.set(States::Game(Game::Pause));
                    if let Some(mut ec) = commands.get_entity(event.target) {
                        ec.insert(resume_sprite.clone());
                    }
                },
            };
        }),
    );

    let pause_btn_sprite = (
        Name::new("pause sprite"),
        SpriteBundle {
            texture: fb_assets.button_pause.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        }
    );

    let get_ready = (
        Name::new("get_ready"),
        SpriteBundle {
            texture: fb_assets.label_get_ready.clone(),
            transform: Transform {
                translation: Vec3::new(0., 60., Z_INDEX_1),
                ..default()
            },
            ..default()
        }
    );

    let score = (
        Name::new("score"),
        SpatialBundle::from_transform(Transform {
            translation: Vec3::new(0., 110., Z_INDEX_1),
            ..default()
        })
    );

    let num_0 = (
        Name::new("num"),
        SpriteBundle {
            texture: fb_assets.number_large_0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        }
    );

    let ground = (
        Name::new("ground"),
        Ground,
        SpriteBundle {
            texture: fb_assets.ground.clone(),
            transform: Transform {
                translation: Vec3::new(12., -100., Z_INDEX_1),
                ..default()
            },
            ..default()
        }
    );

    let guide = (
        Name::new("guide"),
        Guide,
        SpriteBundle {
            texture: fb_assets.instructions.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., Z_INDEX_1),
                ..default()
            },
            ..default()
        }
    );
    
    commands.spawn(bg)
        .with_children(|parent| {
            parent.spawn(pause_btn)
                .with_children(|parent| {
                    parent.spawn(pause_btn_sprite);
                });
            parent.spawn(get_ready);
            parent.spawn(score)
                .with_children(|parent| {
                    parent.spawn(num_0);
                });
            parent.spawn(bird);
            parent.spawn(guide);
            parent.spawn(ground);
        });

    ew_resize.send(ResizeEvent);
}
