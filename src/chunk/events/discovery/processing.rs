use std::sync::{RwLock, RwLockWriteGuard};

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
use enumset::EnumSet;
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
    mut process_list: Local<RwLock<HashSet<Coordinates>>>,
    mut last_time: Local<u128>,
    registry: Res<ChunkRegistry>,
    time: Res<Time>,
) {
    // clear the coordinate process list, we'll do this every 150 milliseconds,
    // less could probably work, but can't really tell too big of a difference.
    if time.elapsed().as_millis() - *last_time >= 150 {
        if let Ok(mut coordinate_queue) = process_list.write() {
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
                let process_list = &mut process_list;

                let result = data
                    .into_par_iter()
                    .flat_map(move |coordinates| {
                        let mut process_list = process_list.write().ok()?;

                        if process_list.contains(&coordinates) {
                            return None;
                        }

                        process_event_data(coordinates, registry, &mut process_list)
                    })
                    .collect::<Vec<_>>();

                return Some(result);
            }
            return None;
        })
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

fn process_event_data(
    coordinates: Coordinates,
    registry: &ChunkRegistry,
    process_list: &mut RwLockWriteGuard<'_, HashSet<IVec3>>,
) -> Option<ProcessWriterType> {
    let Some(chunk) = registry.get_chunk_at(coordinates) else {
        let event = ChunkCreateEvent { coordinates };
        let writer = ProcessWriterType::ChunkCreationWriter(event);

        return Some(writer);
    };

    let result = process_flags(coordinates, chunk.get_flags());

    if let Some(_) = result {
        process_list.insert(coordinates);
    }

    return result;
}

fn process_flags(
    coordinates: Coordinates,
    flags: EnumSet<ChunkFlags>,
) -> Option<ProcessWriterType> {
    if flags.contains(ChunkFlags::Busy) {
        return None;
    }

    if !flags.contains(ChunkFlags::Generated) {
        let event = ChunkGenerateEvent { coordinates };
        let writer = ProcessWriterType::GenerateWriter(event);

        return Some(writer);
    }

    if flags.contains(ChunkFlags::Meshed) && !flags.contains(ChunkFlags::Drawn) {
        let event = ChunkDrawEvent { coordinates };
        let writer = ProcessWriterType::DrawWriter(event);

        return Some(writer);
    }

    if flags.contains(ChunkFlags::Dirty) {
        let event = ChunkMeshEvent { coordinates };
        let writer = ProcessWriterType::MeshWriter(event);

        return Some(writer);
    }

    None
}
