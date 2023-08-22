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
    util::frustum::is_in_frustum_batch,
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
    (center_chunk_x, center_chunk_y, center_chunk_z): (i32, i32, i32),
    (radius, radius_height): (i32, i32),
    (chunk_size, chunk_height): (f32, f32),
    frustum: &Frustum,
) -> Task<Vec<Coordinates>> {
    let pool = AsyncComputeTaskPool::get();
    let spaces = frustum.half_spaces;

    pool.spawn(async move {
        (-radius..=radius)
            .into_par_iter()
            .flat_map(|x_offset| {
                (-radius_height..=radius_height)
                    .flat_map(move |y_offset| {
                        (-radius..=radius)
                            .filter_map(move |z_offset| {
                                let size = ChunkRegistry::CHUNK_SIZE;
                                let height = ChunkRegistry::CHUNK_HEIGHT;

                                let x = (center_chunk_x + x_offset) * chunk_size as i32;
                                let y = (center_chunk_y + y_offset) * chunk_height as i32;
                                let z = (center_chunk_z + z_offset) * chunk_size as i32;

                                let point = Coordinates { x, y, z };

                                let points: [Vec3A; 2] = [
                                    Coordinates {
                                        x: x - (size + 1),
                                        y: y - (height + 1),
                                        z: z - (size + 1),
                                    }
                                    .as_vec3a(),
                                    Coordinates {
                                        x: x + (size + 1),
                                        y: y + (height + 1),
                                        z: z + (size + 1),
                                    }
                                    .as_vec3a(),
                                ];

                                // very simple frustum culling, nothing special.
                                // this does not seem to be completely correct; the corners of the
                                // frustum still seem to get culled, are the half_spaces wrong, or
                                // is something else wrong? it works for now, so whatever!
                                if is_in_frustum_batch::<2>(points, spaces, -4.0)
                                    .iter()
                                    .filter(|result| **result)
                                    .last()
                                    .is_none()
                                {
                                    return None;
                                }

                                Some(point)
                            })
                            .collect::<Vec<Coordinates>>()
                    })
                    .collect::<Vec<Coordinates>>()
            })
            .collect()
    })
}
