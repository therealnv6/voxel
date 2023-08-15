use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;

use super::chunk::Chunk;

pub mod discovery;
pub mod draw;

type MeshQueueEntry = (Handle<Mesh>, (i32, i32));
type ChunkQueueEntry = (Arc<Mutex<Chunk>>, (i32, i32));

#[derive(Resource)]
pub struct ChunkMeshingQueue {
    pub queue: VecDeque<ChunkQueueEntry>,
}

#[derive(Resource)]
pub struct ChunkDrawingQueue {
    pub queue: VecDeque<MeshQueueEntry>,
}

impl ChunkDrawingQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl ChunkMeshingQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}
