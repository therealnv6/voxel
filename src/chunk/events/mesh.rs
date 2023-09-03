use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use crate::chunk::{
    mesh::mesh,
    registry::{ChunkRegistry, Coordinates},
    MeshSettings,
};

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

        if let Some(chunk) = registry.get_chunk_at_mut(coordinates) {
            chunk.set_busy(true);

            let settings = settings.clone();
            let dimensions = chunk.get_dimensions();

            let lod = chunk.get_lod();
            let binding = chunk.get_voxels();

            let task = pool.spawn(async move {
                let voxels = binding.read();

                // this looks a bit shit, but hey it works.
                let value = (
                    mesh(
                        if let Ok(voxels) = voxels {
                            voxels.to_vec()
                        } else {
                            vec![]
                        },
                        lod,
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
) {
    tasks.iter_mut().for_each(|(entity, mut task)| {
        let task = &mut task.0;
        let Some((mesh, coordinates)) = future::block_on(future::poll_once(task)) else {
            return;
        };

        commands.entity(entity).remove::<ChunkMeshTask>();

        let Some(chunk) = registry.get_chunk_at_mut(coordinates) else {
            return;
        };

        let mesh_id = match chunk.get_mesh() {
            Some(handle) => meshes.set(handle, mesh),
            None => meshes.add(mesh),
        };

        chunk.set_mesh(mesh_id);
        chunk.set_busy(false);
    });
}
