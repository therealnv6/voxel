use bevy::prelude::*;
use bevy_tasks::Task;

use super::{draw::ChunkDrawEvent, gen::ChunkGenerateEvent, mesh::ChunkMeshEvent};
use crate::chunk::{event::ChunkCreateEvent, registry::Coordinates};

pub mod processing;
pub mod query;

pub const QUEUE_PROCESS_LIMIT: usize = 20;

#[derive(Event)]
pub struct ChunkDiscoveryEvent;

#[derive(Component)]
pub struct ChunkDiscoveryTask(Task<Vec<Coordinates>>);

pub enum ProcessWriterType {
    MeshWriter(ChunkMeshEvent),
    DrawWriter(ChunkDrawEvent),
    GenerateWriter(ChunkGenerateEvent),
    ChunkCreationWriter(ChunkCreateEvent),
}
