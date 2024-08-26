
use std::time::Duration;

use bevy::{color::palettes::css::{BLACK, BLUE, RED, WHITE}, prelude::*};
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::{events::Click, prelude::On};
use bevy_tweening::{lens::SpriteColorLens, Animator, EaseFunction, Tween, TweenCompleted};
use crate::{components::{bird::BirdBundle, button::PlayBtn, ground::Ground, main_menu::Title, mask::{Mask, MaskCenter, MaskSide}, resize::Resizable, states::InMainMenu, Bg}, constant::{TWEEN_MASK_CENTER_BACK, TWEEN_MENU_TO_GAME, Z_INDEX_1}, events::resize::ResizeEvent, resources::assets::FlappyBirdAssets, states::{Game, States}};
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
            transform: Transform {
                translation: Vec3::new(50., 0., 0.),
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
                translation: Vec3::new(-10., 0., 0.),
                ..default()
            },
            ..default()
        }
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
        On::<Pointer<Click>>::run(|mut q_mask: Query<(Entity, &mut Transform), With<MaskCenter>>, mut commands: Commands, audio: Res<Audio>, fb_assets: Res<FlappyBirdAssets>| {
            audio.play(fb_assets.sfx_swooshing.clone());
            if let Ok((entity, mut transform)) = q_mask.get_single_mut() {
                transform.translation.z = 999.;
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
        }),
        SpriteBundle {
            texture: fb_assets.button_play_normal.clone(),
            transform: Transform {
                translation: Vec3::new(0., -40., Z_INDEX_1),
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

    commands.spawn(bg)
        .with_children(|parent| {
            parent.spawn(title_parent)
                .with_children(|parent| {
                    parent.spawn(label_title);
                    parent.spawn(bird);
                });
            parent.spawn(play);
            parent.spawn(ground);
        });

    ew_resize.send(ResizeEvent);
}

pub fn exit(
    mut commands: Commands,
    q_main_menu: Query<Entity, With<InMainMenu>>,
) {
    info!("main_menu_exit");
    for entity in &q_main_menu {
        if let Some(ec) = commands.get_entity(entity) {
            ec.despawn_recursive();
        }
    }
}

pub fn title_animation(
    time: Res<Time>,
    mut q_title: Query<&mut Transform, With<Title>>,
) {
    if let Ok(mut transform) = q_title.get_single_mut() {
        transform.translation.y = 60. + (time.elapsed_seconds() * 2.).sin() * 2.;
    }
}

pub fn tween_callback_menu_to_game(
    mut reader: EventReader<TweenCompleted>,
    mut next_state: ResMut<NextState<States>>,
) {
    for event in reader.read() {
        if event.user_data == TWEEN_MENU_TO_GAME {
            next_state.set(States::Game(Game::Init));
            
        }
    }
}

pub fn tween_callback_mask_center_back(
    mut reader: EventReader<TweenCompleted>,
    mut q_mask: Query<&mut Transform, With<MaskCenter>>
) {
    for event in reader.read() {
        if event.user_data == TWEEN_MASK_CENTER_BACK {
            if let Ok(mut transform) = q_mask.get_single_mut() {
                transform.translation.z = -1.;
            }
        }
    }
}