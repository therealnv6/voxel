use crate::chunk::registry::Coordinates;

use super::ChunkDrawingQueue;
use bevy::prelude::*;

const CHUNKS_TO_DRAIN: usize = 12;

#[derive(Component)]
pub struct ChunkEntity {
    pub position: Coordinates,
}

/// Draws chunks from the chunk loading queue.
///
/// This function takes a list of chunks from the draw queue and spawns their meshes in the scene.
/// Chunks are drawn using the given `Commands` resource, and after drawing, they are removed from the queue.
///
/// # Parameters
///
/// - `commands`: A `Commands` resource used to issue commands to the Bevy ECS framework for spawning entities.
/// - `queue`: A `ResMut` resource containing the chunk loading queue with meshes and positions.
pub fn draw_chunks(
    mut commands: Commands,
    mut queue: ResMut<ChunkDrawingQueue>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let entries = &mut queue.queue;

    if entries.is_empty() {
        return;
    }

    entries
        .drain(0..entries.len().min(CHUNKS_TO_DRAIN))
        .map(|(mesh, (x, z))| {
            let transform = Transform::from_xyz(x as f32, 0.0, z as f32);
            let bundle = PbrBundle {
                mesh,
                transform,
                material: materials.add(StandardMaterial::default()),
                ..Default::default()
            };

            return (bundle, (x, z));
        })
        .for_each(|(bundle, (x, z))| {
            commands.spawn(bundle).insert(ChunkEntity {
                position: (x, z).into(),
            });
        });
}
