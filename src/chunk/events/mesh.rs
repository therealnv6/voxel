use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use crate::chunk::{
    mesh::mesh,
    registry::{ChunkRegistry, Coordinates},
    voxel::Voxel,
    MeshSettings,
};

use super::draw::ChunkDrawEvent;

#[derive(Event, Clone)]
pub struct ChunkMeshEvent {
    pub coordinates: Coordinates,
}

#[derive(Component)]
pub struct ChunkMeshTask(Task<(Mesh, Coordinates)>);

pub fn mesh_chunk(
    mut commands: Commands,
    mut reader: EventReader<ChunkMeshEvent>,
    mut registry: ResMut<ChunkRegistry>,
    settings: Res<MeshSettings>,
) {
    let pool = AsyncComputeTaskPool::get();

    for event in reader.iter() {
        let ChunkMeshEvent { coordinates } = event;

        let coordinates = *coordinates;
        let adjacent = registry.get_adjacent_chunks(coordinates);

        let adjacent_voxels = adjacent
            .iter()
            .map(|chunk| {
                if let Some(chunk) = chunk {
                    chunk.get_voxels()
                } else {
                    let height = ChunkRegistry::CHUNK_HEIGHT;
                    let width = ChunkRegistry::CHUNK_SIZE;

                    Arc::new(RwLock::new(vec![
                        Voxel {
                            is_solid: true,
                            size: 1.0,
                            color: Color::default()
                        };
                        (width * height * width)
                            .try_into()
                            .expect("Size doesn't fit")
                    ]))
                }
            })
            .collect::<Vec<_>>();

        if let Some(chunk) = registry.get_chunk_at_mut(coordinates) {
            chunk.set_busy(true);

            let settings = settings.clone();

            let dimensions = chunk.get_dimensions();
            let binding = chunk.get_voxels();

            let task = pool.spawn(async move {
                let voxels = binding.read();
                let adjacent_voxels = adjacent_voxels
                    .iter()
                    .flat_map(|voxel_set| voxel_set.read())
                    .map(|voxel_set| voxel_set.to_vec())
                    .collect::<Vec<_>>();

                // this looks a bit shit, but hey it works.
                let value = (
                    mesh(
                        if let Ok(voxels) = voxels {
                            voxels.to_vec()
                        } else {
                            vec![]
                        },
                        adjacent_voxels,
                        settings,
                        dimensions,
                    ),
                    coordinates,
                );

                return value;
            });

            commands.spawn(ChunkMeshTask(task));
        }
    }
}

pub fn process_chunk_meshing(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ChunkMeshTask)>,
    mut registry: ResMut<ChunkRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut writer: EventWriter<ChunkDrawEvent>,
) {
    let mut events = Vec::new();

    tasks.iter_mut().for_each(|(entity, mut task)| {
        let task = &mut task.0;
        let Some((mesh, coordinates)) = future::block_on(future::poll_once(task)) else {
            return;
        };

        commands.entity(entity).remove::<ChunkMeshTask>();

        let Some(chunk) = registry.get_chunk_at_mut(coordinates) else {
            return;
        };

        let mesh_id = meshes.add(mesh);

        chunk.set_mesh(mesh_id.clone());

        chunk.set_dirty(false);
        chunk.set_busy(false);

        events.push(ChunkDrawEvent { coordinates });
    });

    writer.send_batch(events);
}
