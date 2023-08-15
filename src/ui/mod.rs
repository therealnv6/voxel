use bevy::prelude::*;

use bevy_egui::EguiContext;
use bevy_window::PrimaryWindow;
use egui::{Color32, Rounding, Slider};

use crate::chunk::{registry::ChunkRegistry, DiscoverySettings, GenerationSettings, MeshSettings};

pub fn inspector_ui(
    mut commands: Commands,
    mut context: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut mesh_settings: ResMut<MeshSettings>,
    mut gen_settings: ResMut<GenerationSettings>,
    mut discovery_settings: ResMut<DiscoverySettings>,
    pbr_entities: Query<Entity, With<Handle<StandardMaterial>>>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let mut ctx = context.single_mut();
    ctx.get_mut().set_visuals(egui::Visuals {
        panel_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 150),
        ..egui::Visuals::default()
    });

    egui::TopBottomPanel::bottom("hierarchy")
        .default_height(150.0)
        .show(ctx.get_mut(), |ui| {
            egui::SidePanel::left("chunk-settings").show_inside(ui, |ui| {
                ui.heading("Chunk Settings");
                ui.checkbox(&mut mesh_settings.occlusion_culling, "Occlusion Culling");

                ui.add(
                    Slider::new(&mut discovery_settings.discovery_radius, 1..=40)
                        .text("Discovery Radius"),
                );

                if ui.button("Rebuild Chunks").clicked() {
                    // loop over all of the chunks to mark them as dirty
                    chunk_registry
                        // gets all of the chunks
                        .get_all_chunks()
                        .into_iter()
                        // lock all of the chunks, this is then flatly mapped, meaning every lock
                        // result that is not Ok(T) will be disposed.
                        .flat_map(|chunk| chunk.lock())
                        // actually mark them as dirty; this is straightforward.
                        .for_each(|mut chunk| chunk.set_dirty(true));
                }

                if ui.button("Remove PBR Entities").clicked() {
                    pbr_entities.into_iter().for_each(|entity| {
                        commands.entity(entity).despawn();
                    })
                }
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.heading("Generation Settings");

                ui.add(
                    Slider::new(&mut gen_settings.frequency_scale, 0.0..=40.0)
                        .text("Frequency Scale"),
                );
                ui.add(
                    Slider::new(&mut gen_settings.amplitude_scale, 0.0..=40.0)
                        .text("Amplitude Scale"),
                );

                ui.add(Slider::new(&mut gen_settings.threshold, 0.0..=40.0).text("Threshold"));
                ui.add(Slider::new(&mut gen_settings.octaves, 0..=40).text("Octaves"));
                ui.add(Slider::new(&mut gen_settings.persistence, 0.0..=40.0).text("Persistence"));
            });

            ui.allocate_space(ui.available_size());
        });
}
