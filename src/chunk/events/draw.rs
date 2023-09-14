use crate::chunk::{
    registry::{ChunkRegistry, Coordinates},
    ChunkEntity,
};

use bevy::prelude::*;
use bevy_tweening::*;

#[derive(Event)]
pub struct ChunkDrawEvent {
    pub coordinates: Coordinates,
}

pub fn draw_chunks(
    mut commands: Commands,
    mut reader: EventReader<ChunkDrawEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_cache: Local<Option<Handle<StandardMaterial>>>,
    mut registry: ResMut<ChunkRegistry>,
) {
    let material = material_cache.get_or_insert_with(|| materials.add(StandardMaterial::default()));
    let iter = reader.iter();

    for ChunkDrawEvent { coordinates } in iter {
        let Some(chunk) = registry.get_chunk_at_mut(*coordinates) else {
            continue;
        };

        if let Some(mesh) = chunk.get_mesh() {
            if let None = chunk.get_entity() {
                chunk.set_entity(commands.spawn_empty().id());
            }

            let entity = chunk.get_entity().expect("entity not found");
            let mut entity_mut = commands.entity(entity);

            // taken this from my old implementation, is this bad?
            entity_mut
                .remove::<Visibility>()
                .remove::<MaterialMeshBundle<StandardMaterial>>()
                .remove::<Animator<Transform>>()
                .insert((
                    ChunkEntity {
                        position: *coordinates,
                    },
                    MaterialMeshBundle {
                        mesh,
                        material: material.clone_weak(),
                        transform: Transform::from_translation(coordinates.as_vec3()),
                        ..Default::default()
                    },
                ));

            chunk.set_drawn(true);
            chunk.set_busy(false);
        }
    }
}
