use bevy::prelude::{Color, IVec3, UVec3};

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

        let mut chunk = Chunk {
            voxels,
            width,
            height,
            depth,
            world_position,
            generated: false,
            dirty: false,
        };

        return chunk;
    }

    /// Retrieves the voxel on a specified face of another voxel, if present.
    ///
    /// # Parameters
    ///
    /// - `coordinates`: The coordinates of the voxel.
    /// - `face`: The face of the voxel to retrieve.
    ///
    /// # Returns
    ///
    /// A reference to the voxel on the specified face, if within the chunk's bounds.
    pub fn get_voxel_face(&self, coordinates: impl Into<UVec3>, face: VoxelFace) -> Option<&Voxel> {
        let coordinates = coordinates.into();

        let IVec3 {
            mut x,
            mut y,
            mut z,
        } = coordinates.try_into().unwrap();

        match face {
            VoxelFace::Front => z += 1,
            VoxelFace::Back => z -= 1,
            VoxelFace::Left => x -= 1,
            VoxelFace::Right => x += 1,
            VoxelFace::Up => y += 1,
            VoxelFace::Down => y -= 1,
        }

        self.get_voxel((x as u32, y as u32, z as u32))
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
            Some(&self.voxels[index as usize])
        } else {
            None
        }
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

            // we should mark the chunk as dirty, as this will let the systems know the chunk has
            // to get re-mashed.
            self.dirty = true;
        }
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

    pub fn get_dimensions(&self) -> (u32, u32, u32) {
        return (self.width, self.height, self.depth);
    }

    pub fn set_generated(&mut self, gen: bool) {
        self.generated = gen;
    }

    pub fn is_generated(&self) -> bool {
        return self.generated;
    }

    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}
