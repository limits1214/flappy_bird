use std::time::Duration;

use crate::{
    components::{
        game::{BirdBundle, Ground},
        main_menu::{PlayBtn, Title},
        mask::MaskCenter,
        resize::Resizable,
        states::InMainMenu,
    },
    constant::{TWEEN_MASK_CENTER_BACK, TWEEN_MENU_TO_GAME, Z_INDEX_1},
    events::resize::ResizeEvent,
    prelude::{MASK_CENTER_FORE_Z_INDEX, Z_INDEX_0},
    resources::assets::FlappyBirdAssets,
};
use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::{events::Click, prelude::On};
use bevy_tweening::{lens::SpriteColorLens, Animator, EaseFunction, Tween};

pub fn main_enter(
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
            transform: Transform::from_xyz(0., 0., Z_INDEX_0),
            ..default()
        },
    );

    let title_parent = (
        Name::new("title"),
        Title,
        SpatialBundle::from_transform(Transform {
            translation: Vec3::new(0., 60., Z_INDEX_1),
            ..default()
        }),
    );

    let bird = (
        BirdBundle::default(),
        SpriteBundle {
            texture: fb_assets.gen_bird_atlas_texture.clone(),
            transform: Transform::from_xyz(50., 0., 0.),
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
            transform: Transform::from_xyz(-10., 0., 0.),
            ..default()
        },
    );

    let pressed = fb_assets.button_play_pressed.clone();
    let normal = fb_assets.button_play_normal.clone();
    let normal2 = fb_assets.button_play_normal.clone();
    let play = (
        Name::new("play"),
        PlayBtn,
        On::<Pointer<Down>>::target_commands_mut(move |evt, target_commands| {
            target_commands.insert(pressed.clone());
        }),
        On::<Pointer<Up>>::target_commands_mut(move |evt, target_commands| {
            target_commands.insert(normal.clone());
        }),
        On::<Pointer<DragEnd>>::target_commands_mut(move |evt, target_commands| {
            target_commands.insert(normal2.clone());
        }),
        On::<Pointer<Click>>::run(
            |mut q_mask: Query<(Entity, &mut Transform), With<MaskCenter>>,
             mut commands: Commands,
             audio: Res<Audio>,
             fb_assets: Res<FlappyBirdAssets>| {
                audio.play(fb_assets.sfx_swooshing.clone());
                if let Ok((entity, mut transform)) = q_mask.get_single_mut() {
                    transform.translation.z = MASK_CENTER_FORE_Z_INDEX;
                    let transition_tween = Tween::new(
                        EaseFunction::QuarticInOut,
                        Duration::from_millis(500),
                        SpriteColorLens {
                            start: Color::srgba_u8(0, 0, 0, 0),
                            end: BLACK.into(),
                        },
                    )
                    .with_completed_event(TWEEN_MENU_TO_GAME);
                    let transition_tween2 = Tween::new(
                        EaseFunction::QuarticInOut,
                        Duration::from_millis(500),
                        SpriteColorLens {
                            start: BLACK.into(),
                            end: Color::srgba_u8(0, 0, 0, 0),
                        },
                    )
                    .with_completed_event(TWEEN_MASK_CENTER_BACK);

                    let seq = transition_tween.then(transition_tween2);
                    commands.entity(entity).insert(Animator::new(seq));
                }
            },
        ),
        SpriteBundle {
            texture: fb_assets.button_play_normal.clone(),
            transform: Transform::from_xyz(0., -40., Z_INDEX_1),
            ..default()
        },
    );

    let ground = (
        Name::new("ground"),
        Ground,
        SpriteBundle {
            texture: fb_assets.ground.clone(),
            transform: Transform::from_xyz(12., -100., Z_INDEX_1),
            ..default()
        },
    );

    commands.spawn(bg).with_children(|parent| {
        parent.spawn(title_parent).with_children(|parent| {
            parent.spawn(label_title);
            parent.spawn(bird);
        });
        parent.spawn(play);
        parent.spawn(ground);
    });

    ew_resize.send(ResizeEvent);
}

pub fn exit(mut commands: Commands, q_main_menu: Query<Entity, With<InMainMenu>>) {
    info!("main_menu_exit");
    for entity in &q_main_menu {
        if let Some(ec) = commands.get_entity(entity) {
            ec.despawn_recursive();
        }
    }
}
