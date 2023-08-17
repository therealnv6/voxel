use bevy::prelude::{IVec3, UVec3};

use super::{registry::Coordinates, voxel::Voxel};
/// Represents the different faces of a voxel.
#[derive(Debug, Clone, PartialEq)]
pub enum VoxelFace {
    Front,
    Back,
    Left,
    Right,
    Up,
    Down,
}

/// Represents a chunk of voxels in a 3D space.
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    /// The collection of voxels within the chunk.
    pub voxels: Vec<Voxel>,
    /// The width of the chunk in voxels.
    pub width: u32,
    /// The height of the chunk in voxels.
    pub height: u32,
    /// The depth of the chunk in voxels.
    pub depth: u32,
    /// Indicates whether the chunk has been modified and needs an update.
    pub dirty: bool,
    /// Whether the chunk has been generated or not.
    pub generated: bool,
    pub busy: bool,
    /// The world position of the chunk.
    pub world_position: Coordinates,
}

impl Chunk {
    /// Creates a new chunk with the specified dimensions.
    ///
    /// # Parameters
    ///
    /// - `width`: The width of the chunk in voxels.
    /// - `height`: The height of the chunk in voxels.
    /// - `depth`: The depth of the chunk in voxels.
    ///
    /// # Returns
    ///
    /// A new `Chunk` with the provided dimensions and default voxel values.
    pub fn new(width: u32, height: u32, depth: u32, world_position: Coordinates) -> Self {
        // Calculate the total number of voxels in the chunk
        let num_voxels = width * height * depth;

        // Initialize the voxel collection with default values
        let voxels = vec![Voxel::default(); num_voxels as usize];

        let chunk = Chunk {
            voxels,
            width,
            height,
            depth,
            world_position,
            generated: false,
            dirty: false,
            busy: false,
        };

        return chunk;
    }

    /// Retrieves a reference to a voxel at the specified position.
    ///
    /// # Parameters
    ///
    /// - `coordinates`: The coordinates of the voxel.
    ///
    /// # Returns
    ///
    /// A reference to the voxel at the specified position, if within the chunk's bounds.
    pub fn get_voxel(&self, coordinates: impl Into<UVec3>) -> Option<&Voxel> {
        let UVec3 { x, y, z } = coordinates.into();

        if x < self.width && y < self.height && z < self.depth {
            let index = self.get_index([x, y, z]);
            return self.voxels.get(index as usize);
        } else {
            None
        }
    }

    pub fn clone_voxels(&self) -> Vec<Voxel> {
        return self.voxels.clone();
    }

    /// Sets the value of a voxel at the specified position.
    ///
    /// # Parameters
    ///
    /// - `coordinates`: The coordinates of the voxel.
    /// - `voxel`: The new value for the voxel.
    pub fn set_voxel(&mut self, coordinates: impl Into<UVec3>, voxel: Voxel) {
        let UVec3 { x, y, z } = coordinates.into();

        if x < self.width && y < self.height && z < self.depth {
            let index = self.get_index([x, y, z]);
            self.voxels[index as usize] = voxel;
        }
    }

    pub fn set_voxels(&mut self, voxels: impl Into<Vec<Voxel>>) {
        let voxels = voxels.into();
        self.voxels = voxels;
    }

    /// Calculates the linear index in the voxel array for a given 3D position.
    ///
    /// # Parameters
    ///
    /// - `coordinates`: The coordinates of the voxel.
    ///
    /// # Returns
    ///
    /// The linear index corresponding to the specified 3D position.
    fn get_index(&self, coordinates: impl Into<UVec3>) -> u32 {
        let UVec3 { x, y, z } = coordinates.into();

        x + y * self.width + z * self.width * self.height
    }

    /// Retrieves the dimensions of the chunk.
    ///
    /// # Returns
    ///
    /// A tuple `(width, height, depth)` representing the dimensions of the chunk in voxels.
    pub fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.depth)
    }

    /// Sets whether the chunk has been generated.
    ///
    /// # Parameters
    ///
    /// - `gen`: A boolean indicating whether the chunk has been generated.
    pub fn set_generated(&mut self, gen: bool) {
        self.generated = gen;
    }

    /// Checks if the chunk has been generated.
    ///
    /// # Returns
    ///
    /// `true` if the chunk has been generated, `false` otherwise.
    pub fn is_generated(&self) -> bool {
        self.generated
    }

    /// Checks if the chunk has been modified.
    ///
    /// # Returns
    ///
    /// `true` if the chunk has been modified, `false` otherwise.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Sets the dirty status of the chunk.
    ///
    /// # Parameters
    ///
    /// - `dirty`: A boolean indicating whether the chunk is dirty (modified).
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn set_busy(&mut self, busy: bool) {
        self.busy = busy;
    }
}
