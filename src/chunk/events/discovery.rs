use std::sync::{Arc, RwLock};

use bevy::{prelude::*, utils::HashSet};
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use rayon::prelude::*;

use crate::chunk::{
    chunk::ChunkFlags,
    event::ChunkCreateEvent,
    registry::{ChunkRegistry, Coordinates},
    DiscoverySettings,
};

pub const QUEUE_PROCESS_LIMIT: usize = 24;

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

pub enum ProcessWriterType {
    MeshWriter(ChunkMeshEvent),
    DrawWriter(ChunkDrawEvent),
    GenerateWriter(ChunkGenerateEvent),
    ChunkCreationWriter(ChunkCreateEvent),
}

pub fn process_discovery_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ChunkDiscoveryTask)>,
    mut chunk_creation_writer: EventWriter<ChunkCreateEvent>,
    mut generate_writer: EventWriter<ChunkGenerateEvent>,
    mut draw_writer: EventWriter<ChunkDrawEvent>,
    mut mesh_writer: EventWriter<ChunkMeshEvent>,
    mut process_queue: Local<Vec<ProcessWriterType>>,
    // is it worth to use a HashSet for this instead of a Vec?
    coordinate_queue: Local<Arc<RwLock<HashSet<Coordinates>>>>,
    registry: Res<ChunkRegistry>,
) {
    let mut result = tasks
        .iter_mut()
        .flat_map(|(entity, mut task)| {
            if let Some(data) = future::block_on(future::poll_once(&mut task.0)) {
                commands.entity(entity).remove::<ChunkDiscoveryTask>();

                let registry = registry.clone();
                let coordinate_queue = coordinate_queue.clone();

                return Some(
                    data.into_par_iter()
                        .flat_map(
                            move |(
                                IVec3 { x, y, z },
                                // we might need this eventually, but has gone unused for the time
                                // being. don't remove this please!
                                IVec3 {
                                    x: _x_offset,
                                    y: _y_offset,
                                    z: _z_offset,
                                },
                            )| {
                                let chunk = registry.get_chunk_at([x, y, z]);
                                let coordinates = Coordinates::new(x, y, z);

                                let Ok(mut coordinate_queue) = coordinate_queue.write() else {
                                    return None;
                                };

                                if coordinate_queue.contains(&coordinates) {
                                    return None;
                                }

                                let result = match chunk {
                                    Some(chunk) => {
                                        let flags = chunk.get_flags();

                                        if flags.contains(ChunkFlags::Busy) {
                                            None
                                        } else if !flags.contains(ChunkFlags::Generated) {
                                            Some(ProcessWriterType::GenerateWriter(
                                                ChunkGenerateEvent { coordinates },
                                            ))
                                        } else if flags.contains(ChunkFlags::Meshed)
                                            && !flags.contains(ChunkFlags::Drawn)
                                        {
                                            Some(ProcessWriterType::DrawWriter(ChunkDrawEvent {
                                                coordinates,
                                            }))
                                        } else if flags.contains(ChunkFlags::Dirty) {
                                            Some(ProcessWriterType::MeshWriter(ChunkMeshEvent {
                                                coordinates,
                                            }))
                                        } else {
                                            None
                                        }
                                    }
                                    None => {
                                        return Some(ProcessWriterType::ChunkCreationWriter(
                                            ChunkCreateEvent { coordinates },
                                        ));
                                    }
                                };

                                if let Some(_) = result {
                                    coordinate_queue.insert(coordinates);
                                }

                                return result;
                            },
                        )
                        .collect::<Vec<_>>(),
                );
            }
            return None;
        })
        // we simply flatten once to remove the double Vec
        .flatten()
        .collect::<Vec<_>>();

    if !result.is_empty() {
        process_queue.append(&mut result);
    }

    // this slows down chunk loading, but the fps improvement far exceeds it.
    let length = process_queue.len();
    let range = 0..length.min(QUEUE_PROCESS_LIMIT);

    let iter = process_queue.drain(range);

    for writer_type in iter {
        match writer_type {
            ProcessWriterType::GenerateWriter(event) => generate_writer.send(event),
            ProcessWriterType::MeshWriter(event) => mesh_writer.send(event),
            ProcessWriterType::DrawWriter(event) => draw_writer.send(event),
            ProcessWriterType::ChunkCreationWriter(event) => chunk_creation_writer.send(event),
        }
    }
}
