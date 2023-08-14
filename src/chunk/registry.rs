use super::chunk::Chunk;
use bevy::utils::HashMap;

#[derive(Debug, Clone)]
pub struct ChunkRegistry {
    chunks: HashMap<i32, Chunk>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates(pub i32, pub i32);

impl ChunkRegistry {
    const CHUNK_SIZE: usize = 16;
    const CHUNK_GRID_SIZE: i32 = 1024;

    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk_at(&mut self, coordinates: Coordinates) -> &mut Chunk {
        self.chunks
            .entry(Self::domain_to_id(coordinates))
            .or_insert_with(|| {
                Chunk::new(
                    Self::CHUNK_SIZE as u32,
                    Self::CHUNK_SIZE as u32,
                    Self::CHUNK_SIZE as u32,
                )
            })
    }

    pub fn domain_to_id(Coordinates(x, z): Coordinates) -> i32 {
        let linear_x = x / Self::CHUNK_SIZE as i32;
        let linear_z = z / Self::CHUNK_SIZE as i32;
        (linear_x * Self::CHUNK_GRID_SIZE) + linear_z
    }

    pub fn id_to_domain(id: i32) -> [i32; 2] {
        let linear_x = id / Self::CHUNK_GRID_SIZE;
        let linear_z = id % Self::CHUNK_GRID_SIZE;

        [
            linear_x * (Self::CHUNK_SIZE as i32),
            linear_z * (Self::CHUNK_SIZE as i32),
        ]
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

        assert_ne!(
            ChunkRegistry::domain_to_id(Coordinates(17, 15)),
            ChunkRegistry::domain_to_id(Coordinates(15, 15))
        );
    }
}
