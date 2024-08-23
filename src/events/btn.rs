use bevy::prelude::*;
use bevy_mod_picking::{events::{Click, Pointer}, prelude::ListenerInput};

#[derive(Event)]
pub struct PlayBtnClickEvent(Entity, f32);

impl From<ListenerInput<Pointer<Click>>> for PlayBtnClickEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        Self(value.target, value.hit.depth)
    }
}