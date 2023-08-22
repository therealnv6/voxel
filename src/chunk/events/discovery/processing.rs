use std::sync::RwLock;

use crate::chunk::events::discovery::QUEUE_PROCESS_LIMIT;
use crate::chunk::events::draw::ChunkDrawEvent;
use crate::chunk::events::gen::ChunkGenerateEvent;
use crate::chunk::events::mesh::ChunkMeshEvent;
use crate::chunk::{
    chunk::ChunkFlags,
    event::ChunkCreateEvent,
    registry::{ChunkRegistry, Coordinates},
};
use bevy::prelude::*;
use bevy::utils::HashSet;
use futures_lite::future;
use rayon::prelude::*;

use super::{ChunkDiscoveryTask, ProcessWriterType};

pub fn process_discovery_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ChunkDiscoveryTask)>,
    mut chunk_creation_writer: EventWriter<ChunkCreateEvent>,
    mut generate_writer: EventWriter<ChunkGenerateEvent>,
    mut draw_writer: EventWriter<ChunkDrawEvent>,
    mut mesh_writer: EventWriter<ChunkMeshEvent>,
    mut process_queue: Local<Vec<ProcessWriterType>>,
    // is it worth to use a HashSet for this instead of a Vec?
    mut coordinate_queue: Local<RwLock<HashSet<Coordinates>>>,
    mut last_time: Local<u128>,
    registry: Res<ChunkRegistry>,
    time: Res<Time>,
) {
    // clear the coordinate_queue
    if (time.elapsed().as_millis() - *last_time) >= 150 {
        if let Ok(mut coordinate_queue) = coordinate_queue.write() {
            coordinate_queue.clear();
            *last_time = time.elapsed().as_millis();
        }
    }

    let mut result = tasks
        .iter_mut()
        .flat_map(|(entity, mut task)| {
            if let Some(data) = future::block_on(future::poll_once(&mut task.0)) {
                commands.entity(entity).remove::<ChunkDiscoveryTask>();

                let registry = &registry;
                let coordinate_queue = &mut coordinate_queue;

                return Some(
                    data.into_par_iter()
                        .flat_map(move |coordinates| {
                            let chunk = registry.get_chunk_at(coordinates);

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
                        })
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
