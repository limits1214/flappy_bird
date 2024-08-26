use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Down, Pointer},
    prelude::ListenerInput,
};

#[derive(Event)]
pub struct JumpEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Down>>> for JumpEvent {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        Self(value.target, value.hit.depth)
    }
}

#[derive(Event)]
pub struct ScoreUpEvent;

#[derive(Event)]
pub struct ResultEvent;
