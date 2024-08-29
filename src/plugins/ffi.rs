use bevy::prelude::*;
use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};

use crate::{events::ffi::FfiEvent, ffi::set_sender, systems::ffi::greet};

pub struct FfiPlugin;

impl Plugin for FfiPlugin {
    fn build(&self, app: &mut App) {
        app.add_crossbeam_event::<FfiEvent>()
            .add_systems(Update, greet.run_if(on_event::<FfiEvent>()));

        let sender = app
            .world()
            .get_resource::<CrossbeamEventSender<FfiEvent>>()
            .unwrap();

        set_sender(sender.clone());
    }
}
