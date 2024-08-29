use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct JumpPickingEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Down>>> for JumpPickingEvent {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        Self(value.target, value.hit.depth)
    }
}

#[derive(Event)]
pub struct MainToGamePickingEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for MainToGamePickingEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        Self(value.target, value.hit.depth)
    }
}

#[derive(Event)]
pub struct PausePickingEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for PausePickingEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        Self(value.target, value.hit.depth)
    }
}

#[derive(Event)]
pub struct ResultToMainPickingEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for ResultToMainPickingEvent {
    fn from(value: ListenerInput<Pointer<Click>>) -> Self {
        Self(value.target, value.hit.depth)
    }
}

#[derive(Event)]
pub struct ResultToGamePickingEvent;

// impl From<ListenerInput<Pointer<Click>>> for ResultToGamePickingEvent {
//     fn from(value: ListenerInput<Pointer<Click>>) -> Self {
//         Self(value.target, value.hit.depth)
//     }
// }
