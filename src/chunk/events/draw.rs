use crate::chunk::{
    registry::{ChunkRegistry, Coordinates},
    ChunkEntity,
};

use bevy::prelude::*;

#[derive(Event)]
pub struct ChunkDrawEvent {
    pub coordinates: Coordinates,
}

pub fn draw_chunks(
    mut commands: Commands,
    mut reader: EventReader<ChunkDrawEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cached_standard_material: Local<Option<Handle<StandardMaterial>>>,
    mut registry: ResMut<ChunkRegistry>,
) {
    let material = cached_standard_material
        .get_or_insert_with(|| materials.add(StandardMaterial::default()))
        .clone();

    for ChunkDrawEvent {
        coordinates: Coordinates { x, y, z },
    } in reader.iter()
    {
        let (x, y, z) = (*x, *y, *z);
        let Some(chunk) = registry.get_chunk_at_mut([x, y, z]) else {
            continue;
        };

        if chunk.get_mesh().is_none() {}

        let transform = Transform::from_xyz(x as f32, y as f32, z as f32);
        let bundle = PbrBundle {
            transform,
            mesh: chunk.get_mesh().expect("Didn't set mesh properly... what?"),
            material: material.clone(),
            ..Default::default()
        };

        chunk.set_drawn(true);
        commands.spawn(bundle).insert(ChunkEntity {
            position: (x, y, z).into(),
        });
    }
}
