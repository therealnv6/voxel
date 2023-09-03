use super::chunk::Chunk;
use bevy::{
    prelude::{IVec3, Resource},
    utils::HashMap,
};

/// A registry for managing and accessing chunks within a 3D environment.
///
/// This struct provides functionality to create and retrieve chunks based on their coordinates,
/// as well as convert between chunk coordinates and IDs for storage and indexing.
#[derive(Debug, Clone, Resource)]
pub struct ChunkRegistry {
    chunks: HashMap<i32, Chunk>,
}

pub type Coordinates = IVec3;

impl ChunkRegistry {
    pub const CHUNK_SIZE: i32 = 16;
    pub const CHUNK_HEIGHT: i32 = 16;

    pub const CHUNK_GRID_SIZE: i32 = (i32::MAX / 48000) - 5;

    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_adjacent_chunks(&self, Coordinates { x, y, z }: Coordinates) -> [Option<&Chunk>; 6] {
        [
            self.get_chunk_at(Coordinates::new(x + 1, y, z)),
            self.get_chunk_at(Coordinates::new(x - 1, y, z)),
            self.get_chunk_at(Coordinates::new(x, y, z + 1)),
            self.get_chunk_at(Coordinates::new(x, y, z - 1)),
            self.get_chunk_at(Coordinates::new(x, y + 1, z)),
            self.get_chunk_at(Coordinates::new(x, y - 1, z)),
        ]
    }

    #[inline]
    pub fn get_chunk_at(&self, coordinates: impl Into<Coordinates>) -> Option<&Chunk> {
        let coordinates = coordinates.into();
        let chunk_id = Self::domain_to_id(coordinates);

        return self.chunks.get(&chunk_id);
    }

    #[inline]
    pub fn get_chunk_at_mut(&mut self, coordinates: impl Into<Coordinates>) -> Option<&mut Chunk> {
        let coordinates = coordinates.into();
        let chunk_id = Self::domain_to_id(coordinates);

        return self.chunks.get_mut(&chunk_id);
    }

    pub fn push_chunk_at(&mut self, coordinates: impl Into<Coordinates>, chunk: Chunk) {
        let coordinates = coordinates.into();
        let chunk_id = Self::domain_to_id(coordinates);

        self.chunks.entry(chunk_id).or_insert(chunk);
    }

    pub fn get_all_chunks(
        &mut self,
    ) -> bevy::utils::hashbrown::hash_map::ValuesMut<'_, i32, Chunk> {
        return self.chunks.values_mut();
    }

    #[inline]
    pub fn domain_to_id(coordinates: impl Into<Coordinates>) -> i32 {
        let IVec3 { x, y, z } = coordinates.into();

        let linear_x = x / Self::CHUNK_SIZE;
        let linear_y = y / Self::CHUNK_SIZE;
        let linear_z = z / Self::CHUNK_SIZE;

        // Calculate the single index for the 3D coordinates
        (linear_x * Self::CHUNK_GRID_SIZE * Self::CHUNK_GRID_SIZE)
            + (linear_y * Self::CHUNK_GRID_SIZE)
            + linear_z
    }

    #[inline]
    pub fn id_to_domain(id: i32) -> Coordinates {
        let linear_x = id / (Self::CHUNK_GRID_SIZE * Self::CHUNK_GRID_SIZE);
        let linear_y =
            (id % (Self::CHUNK_GRID_SIZE * Self::CHUNK_GRID_SIZE)) / Self::CHUNK_GRID_SIZE;
        let linear_z =
            (id % (Self::CHUNK_GRID_SIZE * Self::CHUNK_GRID_SIZE)) % Self::CHUNK_GRID_SIZE;

        Coordinates::new(
            linear_x * Self::CHUNK_SIZE,
            linear_y * Self::CHUNK_SIZE,
            linear_z * Self::CHUNK_SIZE,
        )
    }

    #[inline]
    pub fn get_chunk_center(coordinates: impl Into<Coordinates>) -> Coordinates {
        let chunk_id = Self::domain_to_id(coordinates);
        let chunk_domain = Self::id_to_domain(chunk_id);

        let center_x = chunk_domain.x + (Self::CHUNK_SIZE / 2);
        let center_y = chunk_domain.y + (Self::CHUNK_SIZE / 2);
        let center_z = chunk_domain.z + (Self::CHUNK_SIZE / 2);

        Coordinates::new(center_x, center_y, center_z)
    }
}

#[cfg(test)]
pub mod test {
    use super::ChunkRegistry;
    use crate::chunk::registry::Coordinates;

    #[test]
    fn test_domain() {
        assert_eq!(
            ChunkRegistry::domain_to_id(Coordinates::new(0, 0, 0)),
            ChunkRegistry::domain_to_id(Coordinates::new(1, 0, 7))
        );

        assert_ne!(
            ChunkRegistry::domain_to_id(Coordinates::new(17, 0, 15)),
            ChunkRegistry::domain_to_id(Coordinates::new(15, 0, 15))
        );
    }
}
