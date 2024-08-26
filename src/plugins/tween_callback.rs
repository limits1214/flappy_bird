use crate::systems::prelude::*;
use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

pub struct TweenCallbackPlugin;

impl Plugin for TweenCallbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tween_callback_menu_to_game,
                tween_callback_mask_center_back,
                tween_callback_death_white,
                tween_callback_result_to_menu,
                tween_callback_mask_center_back,
                tween_callback_panel_up,
                tween_callback_spakle_start,
            )
                .run_if(on_event::<TweenCompleted>()),
        );
    }
}
