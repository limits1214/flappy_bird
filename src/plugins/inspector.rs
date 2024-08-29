use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui, DefaultInspectorConfigPlugin,
};
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

use crate::ffi::{Ffi, FfiGreet};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        //iyes_perf_ui
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(PerfUiBundle::default());
            });

        //egui
        app.add_plugins(EguiPlugin)
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(Update, inspector_ui);
    }
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
        });

        if ui.button("ffi greet").clicked() {
            Ffi::greet();
        }

        #[cfg(any(target_os = "ios", target_os = "android"))]
        {
            use crate::ffi::FfiAd;
            if ui.button("ffi ad show").clicked() {
                Ffi::show();
            }
        }

        #[cfg(target_os = "ios")]
        {
            use bevy::winit::WinitWindows;
            use raw_window_handle::HasWindowHandle;
            let mut q_primary = world.query_filtered::<Entity, With<PrimaryWindow>>();
            let windows = world.non_send_resource::<WinitWindows>();
            let e = q_primary.single(&world);
            let winwrapper = windows.get_window(e).unwrap();
            let wh = winwrapper.window_handle().unwrap();
            let rwh = wh.as_raw();
            if ui.button("rwh test").clicked() {
                Ffi::rwh_test(rwh);
            }
        }
    });
}
