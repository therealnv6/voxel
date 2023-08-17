use bevy::prelude::*;

use crate::chunk::{
    registry::{ChunkRegistry},
    ChunkEntity, DiscoverySettings,
};

/// Unload Distant Chunks System
///
/// This system is responsible for unloading chunks that have moved far enough away from the camera's
/// current position. Chunks that are outside the specified discovery radius will be marked as
/// "dirty" to be re-rendered once they come back within the discovery radius, preventing them from
/// appearing as blank chunks.
///
/// # Parameters
///
/// - `commands`: A mutable reference to the ECS commands buffer, used for removing components from entities.
/// - `registry`: A mutable reference to the `ChunkRegistry`, which manages the storage and retrieval of chunks.
/// - `loaded_chunks`: A query that retrieves loaded chunk entities along with their positions.
/// - `transform`: A query that retrieves the transformation data of the camera entity.
/// - `discovery_settings`: A resource containing settings related to chunk discovery and unloading.
///
/// # Details
///
/// Chunks are managed as entities with associated positions. The camera's current translation is used
/// to calculate its position in chunk space. Each loaded chunk's position is also translated to chunk
/// space. The distance between each chunk's position and the camera's position in chunk space is
/// calculated to determine whether the chunk is outside the discovery radius. If so, the chunk is marked
/// as dirty and its rendering material is removed, causing it to be despawned.
///
pub fn unload_distant_chunks(
    mut commands: Commands,
    mut registry: ResMut<ChunkRegistry>,
    loaded_chunks: Query<(Entity, &ChunkEntity)>,
    transform: Query<&Transform, With<Camera>>,
    discovery_settings: Res<DiscoverySettings>,
) {
    let transform = transform.single();
    let translation = transform.translation;

    for (entity, ChunkEntity { position }) in loaded_chunks.iter() {
        let IVec3 {
            x: pos_x,
            y: pos_y,
            z: pos_z,
        } = position;

        let size = ChunkRegistry::CHUNK_SIZE;
        let height = ChunkRegistry::CHUNK_HEIGHT;

        // these values have to be divided by `size` to get the chunked-distance; we need this
        // distance as the discovery_settings.discovery_radius is measured in chunks; not in
        // blocks.
        let dist_x: f32 = (pos_x / size as i32) as f32;
        let dist_y: f32 = (pos_y / height as i32) as f32;
        let dist_z: f32 = (pos_z / size as i32) as f32;

        // same thing goes for these as for the dist_x and dist_z variables above.
        let trans_x = translation.x / size as f32;
        let trans_y = translation.y / size as f32;
        let trans_z = translation.z / size as f32;

        // calculate the difference between the chunk's position and the camera's position
        let diff_x = (dist_x - trans_x).abs();
        let diff_y = (dist_y - trans_y).abs();
        let diff_z = (dist_z - trans_z).abs();

        if diff_x - 1.0 > discovery_settings.discovery_radius.into()
            || diff_z - 1.0 > discovery_settings.discovery_radius.into()
            || diff_y - 1.0 > discovery_settings.discovery_radius_height.into()
        {
            // we have to re-mark it as dirty as it has to get re-rendered once it's within the
            // discovery radius again. otherwise, it will just appear as a blank chunk.

            let chunk = registry.get_chunk_at_mut([*pos_x, *pos_y, *pos_z]);

            // this should always be the case, otherwise how would it have been added to the loaded
            // chunks list? well, we'll still check because why not.
            if let Some(chunk) = chunk {
                chunk.set_dirty(true);

                // remove the rendering material component to despawn the entity.
                // TODO: perhaps we could change this to only mark it as invisible,
                // that way we don't have to re-mesh the chunk once we enter
                // the discovery radius again.
                commands.entity(entity).despawn();
            }
        }
    }
}
