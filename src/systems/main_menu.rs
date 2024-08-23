
use bevy::prelude::*;
use bevy_mod_picking::{events::Click, prelude::On, PickableBundle};
use crate::{components::{bird::{gen_bird_component, Bird, BirdAnimateTimer, BirdBundle}, button::PlayBtn, ground::Ground, main_menu::Title, resize::Resizable, states::InMainMenu}, events::{btn::PlayBtnClickEvent, resize::ResizeEvent}, resources::assets::FlappyBirdAssets};
use bevy_mod_picking::prelude::*;

pub fn enter(
    mut commands: Commands,
    fb_assets: Res<FlappyBirdAssets>,
    mut ew_resize: EventWriter<ResizeEvent>,
) {
    info!("main_menu_enter");
    let bg = (
        Name::new("bg"),
        InMainMenu,
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
                translation: Vec3::new(50., 0., 10.),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: fb_assets.gen_bird_atlas_layout.clone(),
        },
    );

    let label_title = (
        Name::new("lable_title"),
        SpriteBundle {
            texture: fb_assets.label_flappy_bird.clone(),
            transform: Transform {
                translation: Vec3::new(-10., 0., 10.),
                ..default()
            },
            ..default()
        }
    );

    let play = (
        Name::new("play"),
        PlayBtn,
        // PickableBundle::default(),
        // On::<Pointer<Click>>::send_event::<PlayBtnClickEvent>(),
        SpriteBundle {
            texture: fb_assets.button_play_normal.clone(),
            ..default()
        }
    );

    let ground = (
        Name::new("ground"),
        Ground,
        SpriteBundle {
            texture: fb_assets.ground.clone(),
            transform: Transform {
                translation: Vec3::new(12., -100., 10.),
                ..default()
            },
            ..default()
        }
    );

    commands.spawn(
        bg
    )
    .with_children(|parent| {
        parent.spawn(
            (
                Name::new("title"),
                Title,
                SpatialBundle::from_transform(Transform {
                    translation: Vec3::new(0., 60., 0.),
                    ..default()
                }),
            )
        )
        .with_children(|parent| {
            parent.spawn(
                label_title
            );
            parent.spawn(
                bird
            );
        });

        parent.spawn(
            play
        );

        parent.spawn(
            ground
        );

    });

    ew_resize.send(ResizeEvent);
}

pub fn exit() {
    info!("main_menu_exit");
}

pub fn title_animation(
    time: Res<Time>,
    mut q_title: Query<&mut Transform, With<Title>>,
) {
    if let Ok(mut transform) = q_title.get_single_mut() {
        // info!("{}", (time.elapsed_seconds()).sin());
        transform.translation.y = 60. + (time.elapsed_seconds() * 2.).sin() * 2.;
    }
}