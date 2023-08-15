use once_cell::sync::Lazy;
use parking_lot::{RwLock, RwLockWriteGuard};

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;

use super::chunk::Chunk;

pub mod discovery;
pub mod draw;
pub mod generation;
pub mod unload;

pub static CHUNK_GEN_QUEUE: Lazy<RwLock<ChunkGenerationQueue>> =
    Lazy::new(|| RwLock::new(ChunkGenerationQueue::default()));

pub fn get_generation_queue<'a>() -> RwLockWriteGuard<'a, ChunkGenerationQueue> {
    CHUNK_GEN_QUEUE.write()
}

type MeshQueueEntry = (Handle<Mesh>, (i32, i32));
type ChunkQueueEntry = (Arc<Mutex<Chunk>>, (i32, i32));

#[derive(Resource, Default)]
pub struct ChunkMeshingQueue {
    pub queue: VecDeque<ChunkQueueEntry>,
}

#[derive(Resource, Default)]
pub struct ChunkGenerationQueue {
    pub queue: VecDeque<ChunkQueueEntry>,
}

#[derive(Resource, Default)]
pub struct ChunkDrawingQueue {
    pub queue: VecDeque<MeshQueueEntry>,
}
