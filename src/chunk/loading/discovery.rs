use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use super::ChunkDrawingQueue;
use crate::chunk::{registry::ChunkRegistry, DiscoverySettings, MeshSettings};

#[derive(Component)]
pub struct ComputeMesh(Task<(Option<Mesh>, (i32, i32))>);

/// Loads chunks into the chunk loading queue for rendering.
///
/// This function queries the camera's transform to determine its position and loads the corresponding chunk
/// into the loading queue for rendering. If the chunk is dirty (needs to be re-rendered), its mesh is
/// generated and added to the queue.
///
/// # Parameters
///
/// - `transform`: A query for the camera's transform, used to determine the camera's position.
/// - `meshes`: A `ResMut` resource containing the assets for meshes.
/// - `queue`: A `ResMut` resource representing the chunk loading queue.
/// - `registry`: A `ResMut` resource holding the chunk registry, managing chunk data and state.
pub fn load_chunks(
    mut commands: Commands,
    mut registry: ResMut<ChunkRegistry>,
    mesh_settings: Res<MeshSettings>,
    discovery_settings: Res<DiscoverySettings>,
    transform: Query<&Transform, With<Camera>>,
) {
    let transform = transform.single();
    let translation = transform.translation;

    let chunk_size = ChunkRegistry::CHUNK_SIZE as i32;

    let center_chunk_x = (translation.x / chunk_size as f32) as i32;
    let center_chunk_z = (translation.z / chunk_size as f32) as i32;

    let radius = discovery_settings.discovery_radius as i32;
    let thread_pool = AsyncComputeTaskPool::get();

    (-radius..=radius)
        .flat_map(|x_offset| (-radius..=radius).map(move |z_offset| (x_offset, z_offset)))
        .map(move |(x_offset, z_offset)| {
            let x = (center_chunk_x + x_offset) * chunk_size;
            let z = (center_chunk_z + z_offset) * chunk_size;

            let mesh_settings = mesh_settings.clone();
            let chunk = registry.get_chunk_at([x, z]);

            let task: Task<(Option<Mesh>, (i32, i32))> = thread_pool.spawn(async move {
                let Ok(mut chunk) = chunk.lock() else {
                    return (None, (x, z));
                };

                if !chunk.is_dirty() || !chunk.is_generated() {
                    return (None, (x, z));
                } else {
                    let mesh = chunk.mesh(mesh_settings);
                    let pos = (x, z);

                    chunk.set_dirty(false);

                    return (Some(mesh), pos);
                }
            });
            return task;
        })
        .for_each(|task| {
            commands.spawn(ComputeMesh(task));
        });
}

pub fn handle_mesh_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeMesh)>,
    mut draw_queue: ResMut<ChunkDrawingQueue>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, mut task) in &mut tasks {
        // this honestly looks very hacky and stupid, but this is taken from the bevy examples so i
        // assume it should be right. is there a better way?
        if let Some((mesh, (x, z))) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<ComputeMesh>();

            if mesh.is_none() {
                continue;
            }

            let mesh = mesh.unwrap();
            let handle = meshes.add(mesh);

            draw_queue.queue.push_back((handle, (x, z)));
        }
    }
}
