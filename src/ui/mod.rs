// use std::io::{self, Read, Write};

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use egui::plot::{Line, Plot, PlotPoints};

use bevy_egui::EguiContext;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_window::PrimaryWindow;

pub fn inspector_ui(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::SidePanel::left("hierarchy")
        .default_width(200.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Hierarchy");

                bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                    world,
                    ui,
                    &mut selected_entities,
                );

                ui.label("Press escape to toggle UI");
                ui.allocate_space(ui.available_size());
            });
        });

    egui::SidePanel::right("inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Inspector");

                match selected_entities.as_slice() {
                    &[entity] => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                    }
                    entities => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                            world, entities, ui,
                        );
                    }
                }

                ui.allocate_space(ui.available_size());
            });
        });

    egui::TopBottomPanel::bottom("diagnostics")
        .max_height(300.0)
        .show(egui_context.get_mut(), |ui| {
            let diagnostics = world
                .get_resource::<DiagnosticsStore>()
                .expect("DiagnosticStore is not present, why?");

            for diagnostic in [FrameTimeDiagnosticsPlugin::FPS] {
                if let Some(value) = diagnostics.get(diagnostic) {
                    let smoothed = value.smoothed();

                    if let Some(smoothed) = smoothed {
                        ui.label(format!("FPS: {}", smoothed));
                    }

                    // Add a framerate graph using egui::plot::Plot
                    let values: PlotPoints = value
                        .values()
                        .enumerate()
                        .map(|(i, sample)| [i as f64, *sample])
                        .collect();

                    Plot::new("framerate")
                        .view_aspect(50.0)
                        .width(250.0)
                        .show(ui, |ui| {
                            ui.line(Line::new(values));
                        });
                }
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}
