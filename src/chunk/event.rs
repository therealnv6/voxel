use super::registry::{ChunkRegistry, Coordinates};
use bevy::prelude::*;

#[derive(Event)]
pub struct ChunkCreateEvent {
    pub coordinates: Coordinates,
}

pub fn create_chunk(
    mut reader: EventReader<ChunkCreateEvent>,
    mut registry: ResMut<ChunkRegistry>,
) {
    let iter = reader.iter();
    let length = iter.len();

    registry.reserve_chunks(length);

    for ChunkCreateEvent { coordinates } in iter {
        registry.push_chunk_at(
            *coordinates,
            super::chunk::Chunk::new(
                ChunkRegistry::CHUNK_SIZE as u32,
                ChunkRegistry::CHUNK_HEIGHT as u32,
                ChunkRegistry::CHUNK_SIZE as u32,
                ChunkRegistry::get_chunk_center(*coordinates),
            ),
        )
    }
}
