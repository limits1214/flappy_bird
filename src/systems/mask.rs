use bevy::{color::palettes::css, prelude::*, sprite::MaterialMesh2dBundle};

use crate::{components::{mask::{Mask, MaskParent, MaskSide}, resize::Resizable}, constant::{MASK_Z_INDEX, ORIGINAL_HEIGHT, ORIGINAL_WIDTH}};

pub fn spawn_mask(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mask_small = 200.;
    let mask_large = 600.;
    let mask_z = MASK_Z_INDEX;
    let left = (
        Name::new("left_mask"),
        Mask(MaskSide::Left),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(mask_small, mask_large)).into(),
            transform: Transform {
                translation: Vec3::new(-(ORIGINAL_WIDTH/2. + mask_small/2.), 0., mask_z),
                ..default()
            },
            material: materials.add(Color::from(css::DIM_GRAY)),
            ..default()
        }
    );

    let right = (
        Name::new("right_mask"),
        Mask(MaskSide::Right),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(mask_small, mask_large)).into(),
            transform: Transform {
                translation: Vec3::new(ORIGINAL_WIDTH/2. + mask_small/2., 0., mask_z),
                ..default()
            },
            material: materials.add(Color::from(css::DIM_GRAY)),
            ..default()
        }
    );

    let up = (
        Name::new("up_mask"),
        Mask(MaskSide::Up),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(mask_large, mask_small)).into(),
            transform: Transform {
                translation: Vec3::new(0., ORIGINAL_HEIGHT/2. + mask_small/2., mask_z),
                ..default()
            },
            material: materials.add(Color::from(css::DIM_GRAY)),
            ..default()
        }
    );

    let down = (
        Name::new("down_mask"),
        Mask(MaskSide::Down),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(mask_large, mask_small)).into(),
            transform: Transform {
                translation: Vec3::new(0., -(ORIGINAL_HEIGHT/2. + mask_small/2.), mask_z),
                ..default()
            },
            material: materials.add(Color::from(css::DIM_GRAY)),
            ..default()
        }
    );

    let parent = (
        Name::new("mask_parent"),
        MaskParent,
        Resizable,
        SpatialBundle::from_transform(
            Transform {
                ..default()
            }
        )
    );

    commands.spawn(parent)
        .with_children(|parent| {
            parent.spawn(up);
            parent.spawn(right);
            parent.spawn(down);
            parent.spawn(left);
        });
}