use bevy::prelude::*;

use bevy_egui::EguiContext;
use bevy_window::PrimaryWindow;
use egui::{Color32, Slider};

use crate::chunk::{registry::ChunkRegistry, DiscoverySettings, GenerationSettings, MeshSettings};

pub fn inspector_ui(
    mut commands: Commands,
    mut context: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut meshing: ResMut<MeshSettings>,
    mut generation: ResMut<GenerationSettings>,
    mut discovery: ResMut<DiscoverySettings>,
    directional_light_entities: Query<Entity, With<DirectionalLight>>,
    pbr_entities: Query<Entity, With<Handle<StandardMaterial>>>,
    mut chunk_registry: ResMut<ChunkRegistry>,
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
                ui.checkbox(&mut meshing.occlusion_culling, "Occlusion Culling");
                ui.checkbox(&mut discovery.lod, "Level of Detail")
                    .on_hover_text("Level of Detail is not recommended to be used. \nThere's a high chance it will break any kind of culling due to inproper coordinate calculations.");

                ui.add(
                    Slider::new(&mut discovery.discovery_radius, 1..=40).text("Discovery Radius"),
                );

                ui.add(
                    Slider::new(&mut discovery.discovery_radius_height, 1..=40)
                        .text("Discovery Height Radius"),
                );

                if ui.button("Rebuild Chunks").clicked() {
                    // loop over all of the chunks to mark them as dirty
                    chunk_registry
                        .get_all_chunks()
                        .into_iter()
                        .for_each(|chunk| chunk.set_dirty(true));
                }

                if ui.button("Remove PBR Entities").clicked() {
                    pbr_entities.into_iter().for_each(|entity| {
                        commands.entity(entity).despawn();
                    })
                }
            });

            egui::SidePanel::left("generation-settings").show_inside(ui, |ui| {
                ui.heading("Generation Settings");

                ui.add(
                    Slider::new(&mut generation.frequency_scale, 0.0..=40.0)
                        .text("Frequency Scale"),
                );
                ui.add(
                    Slider::new(&mut generation.amplitude_scale, 0.0..=40.0)
                        .text("Amplitude Scale"),
                );

                ui.add(Slider::new(&mut generation.threshold, 0.0..=40.0).text("Threshold"));
                ui.add(Slider::new(&mut generation.octaves, 0..=40).text("Octaves"));
                ui.add(Slider::new(&mut generation.persistence, 0.0..=40.0).text("Persistence"));
            });

            egui::SidePanel::left("visual-settings").show_inside(ui, |ui| {
                ui.heading("Visual Settings");

                if ui.button("Disable Directional Light").clicked() {
                    for entity in &directional_light_entities {
                        commands.entity(entity).despawn();
                    }
                }
            });

            ui.allocate_space(ui.available_size());
        });
}
