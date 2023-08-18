use crate::chunk::{
    registry::{ChunkRegistry, Coordinates},
    ChunkEntity,
};

use bevy::prelude::*;
use bevy_tweening::Animator;

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

    for ChunkDrawEvent {
        coordinates: Coordinates { x, y, z },
    } in reader.iter()
    {
        let (x, y, z) = (*x, *y, *z);

        let Some(chunk) = registry.get_chunk_at_mut([x, y, z]) else {
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
                        position: (x, y, z).into(),
                    },
                    MaterialMeshBundle {
                        mesh,
                        material: material.clone_weak(),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..Default::default()
                    },
                ));

            chunk.set_drawn(true);
        }
    }
}
