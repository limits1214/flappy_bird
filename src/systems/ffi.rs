use bevy::prelude::*;

use crate::events::{ffi::FfiEvent, picking_callback::ResultToGamePickingEvent};

pub fn greet() {
    info!("ffi greet!!!");
}

pub fn ad_dismissed(
    mut read: EventReader<FfiEvent>,
    mut ew_togame: EventWriter<ResultToGamePickingEvent>,
) {
    for _ in read.read() {
        ew_togame.send(ResultToGamePickingEvent);
    }
}
