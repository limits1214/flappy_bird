use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::{components::{bird::{Bird, BirdBundle}, ground::Ground, guide::Guide, puase::PauseBtn, resize::Resizable, states::InGame, Bg}, constant::{Z_INDEX_1, Z_INDEX_10}, events::{jump::JumpEvent, resize::ResizeEvent}, resources::assets::FlappyBirdAssets, states::Game};
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
        Bg,
        Resizable,
        SpriteBundle {
            texture: fb_assets.background_day.clone(),
            ..default()
        },
        On::<Pointer<Down>>::send_event::<JumpEvent>(),
    );

    let bird = (
        BirdBundle::default(),
        RigidBody::Static,
        Collider::circle(17./2.),
        // LinearDamping(0.0),
        ColliderDensity(0.0),
        Mass(5.0),
        // ColliderDensity(2.5),
        // Mass(5.0),
        // GravityScale(100.),
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
            translation: Vec3::new(-55., 110., Z_INDEX_10),
            ..default()
        }),
        On::<Pointer<Click>>::run(move |
            event: Listener<Pointer<Click>>,
            now_state: Res<State<States>>,
            mut next_state: ResMut<NextState<States>>,
            mut commands: Commands,
            mut time: ResMut<Time<Physics>>
            | {
            info!("pause!!");
            match *now_state.get() {
                States::Game(Game::Pause) => {
                    next_state.set(States::Game(Game::Game));
                    if let Some(mut ec) = commands.get_entity(event.target) {
                        ec.insert(pause_sprite.clone());
                    }
                    time.unpause();
                },
                _ => {
                    next_state.set(States::Game(Game::Pause));
                    if let Some(mut ec) = commands.get_entity(event.target) {
                        ec.insert(resume_sprite.clone());
                    }
                    time.pause();
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
    let ground_collider = (
        RigidBody::Static,
        Collider::rectangle(168., 56.),
        TransformBundle::from_transform(Transform {
            translation: Vec3::new(0., -100., Z_INDEX_1),
            ..default()
        })
    );

    let guide = (
        Name::new("guide"),
        SpriteBundle {
            texture: fb_assets.instructions.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., Z_INDEX_1),
                ..default()
            },
            ..default()
        }
    );

    let guide_parent = (
        Name::new("guide"),
        Guide,
        SpatialBundle {
            ..default()
        }
    );
    
    commands.spawn(bg)
        .with_children(|parent| {
            parent.spawn(pause_btn)
                .with_children(|parent| {
                    parent.spawn(pause_btn_sprite);
                });
            parent.spawn(score)
                .with_children(|parent| {
                    parent.spawn(num_0);
                });
            parent.spawn(guide_parent)
                .with_children(|parent| {
                    parent.spawn(get_ready);
                    parent.spawn(guide);
                });
            parent.spawn(bird);
            parent.spawn(ground);
            parent.spawn(ground_collider);
        });

    ew_resize.send(ResizeEvent);
}
