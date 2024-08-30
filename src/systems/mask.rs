use bevy::{color::palettes::css, prelude::*};

use crate::{
    components::{
        mask::{Mask, MaskCenter, MaskParent, MaskSide},
        resize::Resizable,
    },
    constant::{MASK_Z_INDEX, ORIGINAL_HEIGHT, ORIGINAL_WIDTH},
    prelude::MASK_CENTER_BACK_Z_INDEX,
};

pub fn spawn_mask(mut commands: Commands) {
    let mask_small = 200.;
    let mask_large = 600.;
    let mask_z = MASK_Z_INDEX;
    let left = (
        Name::new("left_mask"),
        // Mask(MaskSide::Left),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(Vec2::new(mask_small, mask_large)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-(ORIGINAL_WIDTH / 2. + mask_small / 2.), 0., mask_z),
                ..default()
            },
            ..default()
        },
    );

    let right = (
        Name::new("right_mask"),
        // Mask(MaskSide::Right),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(Vec2::new(mask_small, mask_large)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(ORIGINAL_WIDTH / 2. + mask_small / 2., 0., mask_z),
                ..default()
            },
            ..default()
        },
    );

    let up = (
        Name::new("up_mask"),
        // Mask(MaskSide::Up),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(Vec2::new(mask_large, mask_small)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., ORIGINAL_HEIGHT / 2. + mask_small / 2., mask_z),
                ..default()
            },
            ..default()
        },
    );

    let down = (
        Name::new("down_mask"),
        // Mask(MaskSide::Down),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(Vec2::new(mask_large, mask_small)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., -(ORIGINAL_HEIGHT / 2. + mask_small / 2.), mask_z),
                ..default()
            },
            ..default()
        },
    );

    let center = (
        Name::new("center_mask"),
        MaskCenter,
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba_u8(0, 0, 0, 0),
                custom_size: Some(Vec2::new(ORIGINAL_WIDTH, ORIGINAL_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., MASK_CENTER_BACK_Z_INDEX),
            ..default()
        },
    );

    let parent = (
        Name::new("mask_parent"),
        // MaskParent,
        Resizable,
        SpatialBundle::default(),
    );

    commands.spawn(parent).with_children(|parent| {
        parent.spawn(center);
        parent.spawn(up);
        parent.spawn(right);
        parent.spawn(down);
        parent.spawn(left);
    });
}
