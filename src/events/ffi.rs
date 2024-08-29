use bevy::prelude::*;

#[derive(Event, Clone)]
pub enum FfiEvent {
    Greet,
    AdDismiss,
}
