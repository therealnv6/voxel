use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard};

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

/// Represents a chunk of voxels in a 3D space.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub voxels: Arc<RwLock<Vec<Voxel>>>,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mesh: Option<Handle<Mesh>>,
    pub flags: EnumSet<ChunkFlags>,
    pub entity: Option<Entity>,
    pub world_position: Coordinates,
    pub lod: u32,
}

impl Chunk {
    pub fn new(width: u32, height: u32, depth: u32, world_position: Coordinates) -> Self {
        let num_voxels = width * height * depth;
        let voxels = vec![Voxel::default(); num_voxels as usize];

        Self {
            voxels: Arc::new(RwLock::new(voxels)),
            width,
            height,
            depth,
            world_position,
            mesh: None,
            lod: 0,
            entity: None,
            flags: enum_set!(),
        }
    }

    pub fn get_voxel(&self, coordinates: impl Into<UVec3>) -> Option<Voxel> {
        let UVec3 { x, y, z } = coordinates.into();
        let index = self.get_index([x, y, z]);

        return self.voxels.read().ok()?.get(index as usize).copied();
    }

    pub fn borrow_voxels(
        &self,
    ) -> Result<RwLockReadGuard<'_, Vec<Voxel>>, PoisonError<RwLockReadGuard<'_, Vec<Voxel>>>> {
        self.voxels.read()
    }

    pub fn get_voxels(&self) -> Arc<RwLock<Vec<Voxel>>> {
        self.voxels.clone()
    }

    pub fn set_voxel(&mut self, coordinates: impl Into<UVec3>, voxel: Voxel) {
        let UVec3 { x, y, z } = coordinates.into();

        if x < self.width && y < self.height && z < self.depth {
            let index = self.get_index([x, y, z]);
            if let Ok(mut write) = self.voxels.write() {
                write[index as usize] = voxel;
            }
        }
    }

    pub fn set_voxels(&mut self, voxels: impl Into<Vec<Voxel>>) {
        self.voxels = Arc::new(RwLock::new(voxels.into()));
    }

    fn get_index(&self, coordinates: impl Into<UVec3>) -> u32 {
        let UVec3 { x, y, z } = coordinates.into();
        x + y * self.width + z * self.width * self.height
    }

    pub fn get_dimensions(&self) -> UVec3 {
        (self.width, self.height, self.depth).into()
    }

    pub fn set_mesh(&mut self, mesh: Handle<Mesh>) {
        self.mesh = Some(mesh);
        self.flags.insert(ChunkFlags::Meshed);
    }

    pub fn get_mesh(&self) -> Option<Handle<Mesh>> {
        self.mesh.as_ref().map(|mesh| mesh.clone_weak())
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
