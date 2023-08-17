use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use crate::chunk::{
    mesh::mesh,
    registry::{ChunkRegistry, Coordinates},
    MeshSettings,
};

use super::draw::ChunkDrawEvent;

#[derive(Event)]
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

    for ChunkMeshEvent { coordinates } in reader.iter() {
        if let Some(chunk) = registry.get_chunk_at_mut(*coordinates) {
            chunk.set_busy(true);

            let coordinates_clone = coordinates.clone();

            // i want to find a way to avoid having to clone the voxels to pass into the mesh
            // function, any ideas?
            let voxels_clone = chunk.clone_voxels();
            let settings_clone = settings.clone();
            let dimensions = chunk.get_dimensions();

            let task = pool.spawn(async move {
                return (
                    mesh(&voxels_clone, settings_clone, dimensions),
                    coordinates_clone,
                );
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

        writer.send(ChunkDrawEvent { coordinates });
    });
}
