use bevy::prelude::*;
use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};

use crate::{
    events::ffi::FfiEvent,
    ffi::{set_sender, Ffi},
    states::{Game, MyStates},
    systems::ffi::{ad_dismissed, greet},
};

pub struct FfiPlugin;

impl Plugin for FfiPlugin {
    fn build(&self, app: &mut App) {
        app.add_crossbeam_event::<FfiEvent>()
            .add_systems(Update, greet.run_if(on_event::<FfiEvent>()))
            .add_systems(
                Update,
                ad_dismissed.run_if(
                    on_event::<FfiEvent>().and_then(in_state(MyStates::Game(Game::Result))),
                ),
            );

        let sender = app
            .world()
            .get_resource::<CrossbeamEventSender<FfiEvent>>()
            .unwrap();

        set_sender(sender.clone());
    }
}
