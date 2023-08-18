use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use rayon::prelude::*;

use crate::chunk::{
    chunk::ChunkFlags,
    event::ChunkCreateEvent,
    registry::{ChunkRegistry, Coordinates},
    DiscoverySettings,
};

use super::{draw::ChunkDrawEvent, gen::ChunkGenerateEvent, mesh::ChunkMeshEvent};

#[derive(Event)]
pub struct ChunkDiscoveryEvent;

#[derive(Component)]
pub struct ChunkDiscoveryTask(Task<Vec<(Coordinates, Coordinates)>>);

pub fn handle_chunk_discovery(
    mut commands: Commands,
    mut last_translation: Local<Option<Vec3>>,
    discovery_settings: Res<DiscoverySettings>,
    transform: Query<&Transform, With<Camera>>,
) {
    let transform = transform.single();
    let translation = transform.translation;

    if last_translation.is_none() {
        *last_translation = Some(translation);
        return;
    }

    let chunk_size = ChunkRegistry::CHUNK_SIZE as f32;
    let chunk_height = ChunkRegistry::CHUNK_HEIGHT as f32;

    let translation_diff = translation - last_translation.unwrap();

    if translation_diff.length_squared() < 1.0 {
        return;
    }

    *last_translation = Some(translation);

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
    );

    commands.spawn(ChunkDiscoveryTask(task));
}

fn spawn_discovery_task(
    (center_chunk_x, center_chunk_y, center_chunk_z): (i32, i32, i32),
    (radius, radius_height): (i32, i32),
    (chunk_size, chunk_height): (f32, f32),
) -> Task<Vec<(Coordinates, Coordinates)>> {
    let pool = AsyncComputeTaskPool::get();
    pool.spawn(async move {
        (-radius..=radius)
            .into_par_iter()
            .flat_map(|x_offset| {
                (-radius_height..=radius_height)
                    .flat_map(move |y_offset| {
                        (-radius..=radius)
                            .filter_map(move |z_offset| {
                                let x = (center_chunk_x + x_offset) * chunk_size as i32;
                                let y = (center_chunk_y + y_offset) * chunk_height as i32;
                                let z = (center_chunk_z + z_offset) * chunk_size as i32;

                                Some((
                                    Coordinates { x, y, z },
                                    Coordinates::new(x_offset, y_offset, z_offset),
                                ))
                            })
                            .collect::<Vec<(Coordinates, Coordinates)>>()
                    })
                    .collect::<Vec<(Coordinates, Coordinates)>>()
            })
            .collect()
    })
}

pub fn process_discovery_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ChunkDiscoveryTask)>,
    mut chunk_creation_writer: EventWriter<ChunkCreateEvent>,
    mut generate_writer: EventWriter<ChunkGenerateEvent>,
    mut draw_writer: EventWriter<ChunkDrawEvent>,
    mut mesh_writer: EventWriter<ChunkMeshEvent>,
    mut registry: ResMut<ChunkRegistry>,
    discovery_settings: Res<DiscoverySettings>,
) {
    tasks.iter_mut().for_each(|(entity, mut task)| {
        if let Some(data) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<ChunkDiscoveryTask>();

            for (
                IVec3 { x, y, z },
                IVec3 {
                    x: x_offset,
                    y: y_offset,
                    z: z_offset,
                },
            ) in data
            {
                let chunk = registry.get_chunk_at_mut([x, y, z]);
                let coordinates = Coordinates::new(x, y, z);

                match chunk {
                    Some(chunk) => {
                        if discovery_settings.lod {
                            chunk.set_lod({
                                if x_offset >= y_offset && x_offset >= z_offset {
                                    x_offset
                                } else if y_offset >= x_offset && y_offset >= z_offset {
                                    y_offset
                                } else {
                                    z_offset
                                }
                            } as u32);
                        }

                        let flags = chunk.get_flags();

                        if flags.contains(ChunkFlags::Busy) {
                            continue;
                        }

                        if !flags.contains(ChunkFlags::Generated) {
                            generate_writer.send(ChunkGenerateEvent { coordinates });
                        }

                        if flags.contains(ChunkFlags::Meshed) && !flags.contains(ChunkFlags::Drawn)
                        {
                            draw_writer.send(ChunkDrawEvent { coordinates });
                        } else if flags.contains(ChunkFlags::Dirty) {
                            mesh_writer.send(ChunkMeshEvent { coordinates });
                        }
                    }
                    None => chunk_creation_writer.send(ChunkCreateEvent { coordinates }),
                }
            }
        }
    });
}
