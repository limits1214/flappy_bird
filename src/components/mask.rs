use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Mask(pub MaskSide);

#[derive(Debug, PartialEq)]
pub enum MaskSide {
    Left, Up, Right, Down
}

#[derive(Component)]
pub struct MaskParent;

#[derive(Component)]
pub struct MaskCenter;