use crate::chunk::{registry::Coordinates, ChunkEntity};

use bevy::prelude::*;

#[derive(Event)]
pub struct ChunkDrawEvent {
    pub mesh: Handle<Mesh>,
    pub position: Coordinates,
}

pub fn draw_chunks(
    mut commands: Commands,
    mut reader: EventReader<ChunkDrawEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cached_standard_material: Local<Option<Handle<StandardMaterial>>>,
) {
    let material = cached_standard_material
        .get_or_insert_with(|| materials.add(StandardMaterial::default()))
        .clone();

    for ChunkDrawEvent {
        mesh,
        position: Coordinates(x, z),
    } in reader.iter()
    {
        let (x, z) = (*x, *z);
        let transform = Transform::from_xyz(x as f32, 0.0, z as f32);
        let bundle = PbrBundle {
            transform,
            mesh: mesh.clone(),
            material: material.clone(),
            ..Default::default()
        };

        commands.spawn(bundle).insert(ChunkEntity {
            position: (x, z).into(),
        });
    }
}
