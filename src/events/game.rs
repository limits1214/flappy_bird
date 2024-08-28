use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Down, Pointer},
    prelude::ListenerInput,
};

#[derive(Event)]
pub struct ScoreUpEvent;

#[derive(Event)]
pub struct ResultEvent;
