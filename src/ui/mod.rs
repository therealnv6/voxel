use bevy::prelude::*;

use bevy_egui::EguiContext;
use bevy_window::PrimaryWindow;
use egui::Slider;

use crate::chunk::{registry::ChunkRegistry, DiscoverySettings, MeshSettings};

pub fn inspector_ui(
    mut commands: Commands,
    mut context: Query<&mut EguiContext, With<PrimaryWindow>>,
    pbr_entities: Query<Entity, With<Handle<StandardMaterial>>>,
    mut mesh_settings: ResMut<MeshSettings>,
    mut discovery_settings: ResMut<DiscoverySettings>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let mut ctx = context.single_mut();

    egui::TopBottomPanel::bottom("hierarchy")
        .default_height(150.0)
        .show(ctx.get_mut(), |ui| {
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

            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                ui.label("Press Escape to toggle UI");
                ui.label("Press LeftAlt to toggle mouse");
            });

            ui.allocate_space(ui.available_size());
        });
}
