use super::chunk::Chunk;
use bevy::{prelude::Resource, utils::HashMap};

/// A registry for managing and accessing chunks within a 3D environment.
///
/// This struct provides functionality to create and retrieve chunks based on their coordinates,
/// as well as convert between chunk coordinates and IDs for storage and indexing.
#[derive(Debug, Clone, Resource)]
pub struct ChunkRegistry {
    chunks: HashMap<i32, Chunk>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates(pub i32, pub i32);

impl ChunkRegistry {
    pub const CHUNK_SIZE: usize = 16;
    pub const CHUNK_HEIGHT: usize = 128;
    pub const CHUNK_GRID_SIZE: i32 = 1024;

    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    /// Retrieves an Arc wrapped in a Mutex for the chunk at the specified coordinates.
    ///
    /// If the chunk does not exist at the given coordinates, a new chunk is created, added to
    /// the registry, and then returned.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - The 3D coordinates of the requested chunk.
    ///
    /// # Returns
    ///
    /// An Arc wrapped in a Mutex, representing the requested chunk.
    pub fn get_chunk_at(&self, coordinates: impl Into<Coordinates>) -> Option<&Chunk> {
        let coordinates = coordinates.into();
        let chunk_id = Self::domain_to_id(coordinates);

        return self.chunks.get(&chunk_id);
    }

    pub fn get_chunk_at_mut(&mut self, coordinates: impl Into<Coordinates>) -> Option<&mut Chunk> {
        let coordinates = coordinates.into();
        let chunk_id = Self::domain_to_id(coordinates);

        return self.chunks.get_mut(&chunk_id);
    }

    pub fn push_chunk_at(&mut self, coordinates: impl Into<Coordinates>, chunk: Chunk) {
        let coordinates = coordinates.into();
        let chunk_id = Self::domain_to_id(coordinates);

        self.chunks.insert(chunk_id, chunk);
    }

    /// Retrieves an iterator over all the chunks stored in the registry.
    ///
    /// # Returns
    ///
    /// An iterator over the stored chunks.
    pub fn get_all_chunks(
        &mut self,
    ) -> bevy::utils::hashbrown::hash_map::ValuesMut<'_, i32, Chunk> {
        return self.chunks.values_mut();
    }

    /// Converts 3D chunk coordinates to a unique identifier.
    ///
    /// The conversion process involves mapping the 3D coordinates to a linear space based on
    /// chunk size and grid size, resulting in a single ID.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - The 3D coordinates of the chunk.
    ///
    /// # Returns
    ///
    /// A unique identifier for the chunk.
    pub fn domain_to_id(coordinates: impl Into<Coordinates>) -> i32 {
        let Coordinates(x, z) = coordinates.into();

        let linear_x = x / Self::CHUNK_SIZE as i32;
        let linear_z = z / Self::CHUNK_SIZE as i32;
        (linear_x * Self::CHUNK_GRID_SIZE) + linear_z
    }

    /// Converts an ID to the 3D coordinates of the corresponding chunk.
    ///
    /// This function reverses the process of `domain_to_id`, deriving the chunk's coordinates
    /// from its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the chunk.
    ///
    /// # Returns
    ///
    /// An array containing the X and Z coordinates of the chunk.
    pub fn id_to_domain(id: i32) -> [i32; 2] {
        let linear_x = id / Self::CHUNK_GRID_SIZE;
        let linear_z = id % Self::CHUNK_GRID_SIZE;

        [
            linear_x * (Self::CHUNK_SIZE as i32),
            linear_z * (Self::CHUNK_SIZE as i32),
        ]
    }

    /// Computes the center coordinates of a chunk based on its 3D coordinates.
    ///
    /// This function calculates the center of a chunk in the world space using the chunk's
    /// coordinates and size.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - The 3D coordinates of the chunk.
    ///
    /// # Returns
    ///
    /// The center coordinates of the chunk.
    pub fn get_chunk_center(coordinates: impl Into<Coordinates>) -> Coordinates {
        let chunk_id = Self::domain_to_id(coordinates);
        let chunk_domain = Self::id_to_domain(chunk_id);

        let center_x = chunk_domain[0] + (Self::CHUNK_SIZE as i32 / 2);
        let center_z = chunk_domain[1] + (Self::CHUNK_SIZE as i32 / 2);

        Coordinates(center_x, center_z)
    }
}

impl Into<Coordinates> for i32 {
    fn into(self) -> Coordinates {
        let [x, z] = ChunkRegistry::id_to_domain(self);
        Coordinates(x, z)
    }
}

impl Into<Coordinates> for (i32, i32) {
    fn into(self) -> Coordinates {
        Coordinates(self.0, self.1)
    }
}

impl Into<Coordinates> for [i32; 2] {
    fn into(self) -> Coordinates {
        Coordinates(self[0], self[1])
    }
}

#[cfg(test)]
pub mod test {
    use super::ChunkRegistry;
    use crate::chunk::registry::Coordinates;

    #[test]
    fn test_domain() {
        assert_eq!(
            ChunkRegistry::domain_to_id(Coordinates(0, 0)),
            ChunkRegistry::domain_to_id(Coordinates(1, 7))
        );

        assert_eq!(
            ChunkRegistry::domain_to_id(Coordinates(0, 0)),
            ChunkRegistry::domain_to_id(ChunkRegistry::domain_to_id(Coordinates(0, 0))),
        );

        assert_ne!(
            ChunkRegistry::domain_to_id(Coordinates(17, 15)),
            ChunkRegistry::domain_to_id(Coordinates(15, 15))
        );
    }
}
