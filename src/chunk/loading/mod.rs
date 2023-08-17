use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

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

// unlike the other queues, this queue is static; this is because we are accessing it from chunk
// instantiation. if we'd have to add them to another queue and handle it there, i feel like it'd
// be too much overhead.
static mut CHUNK_GEN_QUEUE: Lazy<ChunkGenerationQueue> =
    Lazy::new(|| ChunkGenerationQueue::default());

// this is just a simple helper method for accessing the generation queue. use this instead of
// accessing the CHUNK_GEN_QUEUE manually!
pub fn get_generation_queue<'a>() -> &'a mut ChunkGenerationQueue {
    unsafe { Lazy::force_mut(&mut CHUNK_GEN_QUEUE) }
}

pub type MeshQueueEntry = (Handle<Mesh>, (i32, i32));
pub type ChunkQueueEntry = (Arc<Mutex<Chunk>>, (i32, i32));

#[derive(Resource, Default)]
pub struct ChunkMeshingQueue {
    pub queue: VecDeque<ChunkQueueEntry>,
}

#[derive(Resource, Default)]
pub struct ChunkGenerationQueue {
    pub queue: SegQueue<ChunkQueueEntry>,
}

#[derive(Resource, Default)]
pub struct ChunkDrawingQueue {
    pub queue: VecDeque<MeshQueueEntry>,
}
