use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Mask(pub MaskSide);

#[derive(Debug)]
pub enum MaskSide {
    Left, Up, Right, Down
}

#[derive(Component)]
pub struct MaskParent;