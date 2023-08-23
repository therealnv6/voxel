use bevy::{
    math::Vec3A,
    prelude::*,
    render::primitives::{Frustum, HalfSpace},
};
use bevy_tasks::{AsyncComputeTaskPool, Task};

use rayon::prelude::*;

use crate::{
    chunk::{
        registry::{ChunkRegistry, Coordinates},
        DiscoverySettings,
    },
    util::frustum::{create_frustum_points, is_in_frustum_batch, is_in_frustum_batch_unsized},
};

use super::ChunkDiscoveryTask;

pub fn handle_chunk_discovery(
    mut commands: Commands,
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
        &frustum,
    );

    commands.spawn(ChunkDiscoveryTask(task));
}

fn spawn_discovery_task(
    center_chunk: (i32, i32, i32),
    radius: (i32, i32),
    chunk_sizes: (f32, f32),
    frustum: &Frustum,
) -> Task<Vec<Coordinates>> {
    let pool = AsyncComputeTaskPool::get();
    let spaces = frustum.half_spaces;

    pool.spawn(async move {
        (-radius.0..=radius.0)
            .flat_map(|x_offset| {
                (-radius.1..=radius.1).flat_map(move |y_offset| {
                    (-radius.0..=radius.0).filter_map(move |z_offset| {
                        let chunk_size = chunk_sizes.0 as i32;
                        let chunk_height = chunk_sizes.1 as i32;
                        let size = ChunkRegistry::CHUNK_SIZE;
                        let height = ChunkRegistry::CHUNK_HEIGHT;

                        let x = (center_chunk.0 + x_offset) * chunk_size;
                        let y = (center_chunk.1 + y_offset) * chunk_height;
                        let z = (center_chunk.2 + z_offset) * chunk_size;

                        let point = Coordinates { x, y, z };
                        let points =
                            create_frustum_points((x, y, z).into(), (size, height, size).into());

                        // very simple frustum culling, nothing special.
                        // this does not seem to be completely correct; the corners of the
                        // frustum still seem to get culled, are the half_spaces wrong, or
                        // is something else wrong? it works for now, so whatever!
                        if is_in_frustum_batch_unsized(points, spaces, 0.0)
                            .iter()
                            .filter(|result| **result)
                            .next()
                            .is_none()
                        {
                            return None;
                        }

                        Some(point)
                    })
                })
            })
            .collect()
    })
}
