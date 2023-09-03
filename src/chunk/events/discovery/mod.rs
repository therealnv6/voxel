use bevy::{prelude::*, utils::HashSet};
use bevy_tasks::Task;

use super::{draw::ChunkDrawEvent, gen::ChunkGenerateEvent, mesh::ChunkMeshEvent};
use crate::chunk::{event::ChunkCreateEvent, registry::Coordinates};

pub mod processing;
pub mod query;

// this variable is NOT the amount of chunks that get processed in the discovery task, instead,
// it's the amount of chunks that get processed AFTER the discovery task; the results of the
// discovery task.
//
// lower = slower chunk processing, but significantly better performance.
// higher = faster chunk processing, but significantly worse performance.
//
// the performance hit is mostly noticeable when having to process a lot of chunks are added to the
// queue at the same time, for example, if you suddenly move into a section of the world where no
// chunks have been loaded yet.
pub const QUEUE_PROCESS_LIMIT: usize = 20;

#[derive(Event)]
pub struct ChunkDiscoveryEvent;

#[derive(Component)]
pub struct ChunkDiscoveryTask(Task<Vec<Coordinates>>);

/// This is a list of chunks that are marked as "Busy", however this is not to be confused with
/// ChunkFlags::Busy, as this is only for the discovery of chunks, specifically in the case where
/// we don't have access (or don't want to) access the chunk itself to read the flags.
#[derive(Resource)]
pub struct BusyLocations(pub HashSet<Coordinates>);

pub enum ProcessWriterType {
    MeshWriter(ChunkMeshEvent),
    DrawWriter(ChunkDrawEvent),
    GenerateWriter(ChunkGenerateEvent),
    ChunkCreationWriter(ChunkCreateEvent),
}
