use bevy::{prelude::*, render::primitives::Frustum};
use bevy_tasks::{AsyncComputeTaskPool, Task};

use crate::{
    chunk::{
        registry::{ChunkRegistry, Coordinates},
        BusyLocations, DiscoverySettings,
    },
    util::frustum::{create_frustum_points, is_in_frustum_batch_unsized},
};

use super::ChunkDiscoveryTask;

pub fn handle_chunk_discovery(
    mut commands: Commands,
    mut busy: ResMut<BusyLocations>,
    discovery_settings: Res<DiscoverySettings>,
    transform: Query<(&Transform, &Frustum)>,
) {
    let (transform, frustum) = transform.single();

    let translation = transform.translation;

    let chunk_size = ChunkRegistry::CHUNK_SIZE as f32;
    let chunk_height = ChunkRegistry::CHUNK_HEIGHT as f32;

    let center_chunk_x = (translation.x / chunk_size) as i32;
    let center_chunk_y = (translation.y / chunk_height) as i32;
    let center_chunk_z = (translation.z / chunk_size) as i32;

    let (radius, radius_height) = (
        discovery_settings.discovery_radius as i32,
        discovery_settings.discovery_radius_height as i32,
    );

    let task = spawn_discovery_task(
        (center_chunk_x, center_chunk_y, center_chunk_z),
        (radius, radius_height),
        (chunk_size, chunk_height),
        &mut busy,
        &frustum,
    );

    commands.spawn(ChunkDiscoveryTask(task));
}

fn spawn_discovery_task(
    center_chunk: (i32, i32, i32),
    radius: (i32, i32),
    chunk_sizes: (f32, f32),
    locations: &mut BusyLocations,
    frustum: &Frustum,
) -> Task<Vec<Coordinates>> {
    let pool = AsyncComputeTaskPool::get();
    let spaces = frustum.half_spaces;
    let radius_squared = radius.0.pow(2);

    // we clone here to not write the "busy" coordinates into the global set, while still being
    // able to read from the global set. writing this to the global set would interfere with the
    // processing task.
    let mut local_busy = locations.0.clone();

    pool.spawn(async move {
        let mut result = Vec::new();

        for x_offset in -radius.0..=radius.0 {
            for z_offset in -radius.0..=radius.0 {
                for y_offset in -radius.1..=radius.1 {
                    if x_offset * x_offset + z_offset * z_offset >= radius_squared {
                        continue;
                    }

                    let chunk_size = chunk_sizes.0 as i32;
                    let chunk_height = chunk_sizes.1 as i32;

                    let x = (center_chunk.0 + x_offset) * chunk_size;
                    let y = (center_chunk.1 + y_offset) * chunk_height;
                    let z = (center_chunk.2 + z_offset) * chunk_size;

                    let point = Coordinates { x, y, z };

                    if local_busy.contains(&point) {
                        continue;
                    }

                    let points = create_frustum_points(
                        point,
                        (
                            ChunkRegistry::CHUNK_SIZE,
                            ChunkRegistry::CHUNK_HEIGHT,
                            ChunkRegistry::CHUNK_SIZE,
                        )
                            .into(),
                    );

                    if is_in_frustum_batch_unsized(points, spaces)
                        .iter()
                        .any(|result| *result)
                    {
                        result.push(point);
                        local_busy.insert(point);
                    }
                }
            }
        }

        result
    })
}
