use std::sync::Arc;

use bevy::prelude::{Entity, Handle, Mesh, UVec3};
use enumset::{enum_set, EnumSet, EnumSetType};

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

/// Represents the flags that can be associated with a chunk.
#[derive(EnumSetType, Debug)]
pub enum ChunkFlags {
    Generated,
    Dirty,
    Drawn,
    Busy,
    Meshed,
}

#[derive(Debug, Copy, Clone)]
pub struct ChunkDimensions {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// Represents a chunk of voxels in a 3D space.
///
/// A `Chunk` is a fundamental unit of a 3D voxel space. It contains voxel data, mesh information,
/// flags, entity details, position, and level of detail (LOD) information.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
///
/// // Create a new empty chunk, with a size of 16x16x16 at the "global" position 0, 0, 0.
/// let empty_chunk = Chunk::new(
///     16,
///     16,
///     16,
///     Coordinates {
///         0,
///         0,
///         0
///     }
/// );
/// ```
///
/// # Fields
///
/// - `voxels`: An `Arc` (atomic reference-counted) vector of `Voxel` instances. This field is
///   used to store the voxel data for the chunk efficiently, as it can be shared among threads
///   without cloning the data.
///
/// - `dimensions`: A `ChunkDimensions` struct that defines the size and shape of the chunk. This
///   is created using the provided (width, height, depth)
///
/// - `mesh`: An optional `Handle<Mesh>` representing the mesh associated with this chunk. This
///   gets re-used if the chunk is not dirty, but has to get re-rendered.
///
/// - `flags`: An `EnumSet<ChunkFlags>` that contains flags to control various behaviors and
///   properties of the chunk.
///
/// - `entity`: An optional `Entity` representing an entity in the game engine. This field is used
///   to associate the chunk with an entity for rendering and gameplay purposes.
///
/// - `world_position`: The world position of the chunk, represented as `Coordinates`. This is the
///   position of the chunk within the 3D world.
///
/// - `lod`: The level of detail (LOD) of the chunk, represented as a `u32`. LOD is used to control
///   the rendering detail of the chunk, with lower values indicating higher (or lower, can't
///   remember) detail.
///
/// # Thread Safety
///
/// The use of `Arc` for the `voxels` field ensures that the voxel data can be safely shared among
/// multiple threads without the need for cloning.
///
/// # Notes
///
/// - It is not recommended to use `Chunk#set_voxel()` manually in the case of multiple updates
///   being sent, instead, it's recommended to use `Chunk#set_voxels`, as this completely overrides
///   the `voxels` field. See [`set_voxels()`].
///
/// # See Also
///
/// - [`Voxel`](struct.Voxel.html): The individual voxel data structure.
/// - [`ChunkDimensions`](struct.ChunkDimensions.html): Information about the size and shape of a chunk.
/// - [`ChunkFlags`](enum.ChunkFlags.html): Flags to control chunk properties and behaviors.
/// - [`Mesh`](struct.Mesh.html): Represents a mesh for rendering.
/// - [`Entity`](https://bevyengine.org/0.5.0/bevy/ecs/struct.Entity.html): Bevy's entity type for
///   gameplay and rendering.
/// - [`Coordinates`](struct.Coordinates.html): Represents 3D coordinates in the world space.
#[derive(Debug, Clone)]
pub struct Chunk {
    // this is an Arc<T> to avoid cloning; as we pass this into a new thread.
    pub voxels: Arc<Vec<Voxel>>,
    pub dimensions: ChunkDimensions,
    pub mesh: Option<Handle<Mesh>>,
    pub flags: EnumSet<ChunkFlags>,
    // keep track of the current entity to avoid spawning new entities for every respawn
    // this is used to render the entity, by inserting the material components through bevy.
    pub entity: Option<Entity>,
    pub world_position: Coordinates,
    pub lod: u32,
}

impl Chunk {
    pub fn new(width: u32, height: u32, depth: u32, world_position: Coordinates) -> Self {
        let num_voxels = width * height * depth;
        let voxels = vec![Voxel::default(); num_voxels as usize];

        Self {
            voxels: Arc::new(voxels),
            dimensions: ChunkDimensions {
                width,
                height,
                depth,
            },
            world_position,
            mesh: None,
            lod: 0,
            entity: None,
            flags: enum_set!(),
        }
    }

    pub fn get_voxel(&self, coordinates: impl Into<UVec3>) -> Option<&Voxel> {
        let UVec3 { x, y, z } = coordinates.into();
        let index = self.get_index([x, y, z]);

        return self.voxels.get(index as usize);
    }

    pub fn get_voxels<'a>(&self) -> &Vec<Voxel> {
        &self.voxels
    }

    pub fn set_voxel(&mut self, coordinates: impl Into<UVec3>, voxel: Voxel) {
        let UVec3 { x, y, z } = coordinates.into();
        let ChunkDimensions {
            width,
            height,
            depth,
        } = self.dimensions;

        if x < width && y < height && z < depth {
            let index = self.get_index([x, y, z]);
            let mut_data = Arc::get_mut(&mut self.voxels);

            if let Some(value) = mut_data {
                value[index as usize] = voxel;
            }
        }
    }

    pub fn set_voxels(&mut self, voxels: impl Into<Vec<Voxel>>) {
        self.voxels = Arc::new(voxels.into());
    }

    fn get_index(&self, coordinates: impl Into<UVec3>) -> u32 {
        let UVec3 { x, y, z } = coordinates.into();
        let ChunkDimensions { width, height, .. } = self.dimensions;

        x + y * width + z * width * height
    }

    pub fn get_dimensions<'a>(&'a self) -> &'a ChunkDimensions {
        return &self.dimensions;
    }

    pub fn set_mesh(&mut self, mesh: Handle<Mesh>) {
        self.mesh = Some(mesh);
        self.flags.insert(ChunkFlags::Meshed);
    }

    pub fn get_mesh(&self) -> Option<Handle<Mesh>> {
        self.mesh.as_ref().map(|mesh| mesh.clone())
    }

    pub fn get_entity(&self) -> Option<Entity> {
        return self.entity;
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn is_generated(&self) -> bool {
        self.flags.contains(ChunkFlags::Generated)
    }

    pub fn is_dirty(&self) -> bool {
        self.flags.contains(ChunkFlags::Dirty)
    }

    pub fn is_busy(&self) -> bool {
        self.flags.contains(ChunkFlags::Busy)
    }

    pub fn is_drawn(&self) -> bool {
        self.flags.contains(ChunkFlags::Drawn)
    }

    pub fn set_flag(&mut self, flag: ChunkFlags, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    pub fn set_generated(&mut self, gen: bool) {
        self.set_flag(ChunkFlags::Generated, gen);
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.set_flag(ChunkFlags::Dirty, dirty);
    }

    pub fn set_busy(&mut self, busy: bool) {
        self.set_flag(ChunkFlags::Busy, busy);
    }

    pub fn set_drawn(&mut self, drawn: bool) {
        self.set_flag(ChunkFlags::Drawn, drawn);
    }

    pub fn apply_mask(&mut self, flags: EnumSet<ChunkFlags>) {
        self.flags ^= flags;
    }

    pub fn get_flags(&self) -> EnumSet<ChunkFlags> {
        self.flags
    }

    pub fn set_lod(&mut self, lod: u32) {
        self.lod = lod;
    }

    pub fn get_lod(&mut self) -> u32 {
        return self.lod;
    }
}
