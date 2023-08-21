use bevy::prelude::Color;
use half::f16;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Voxel {
    pub color: Color,
    pub is_solid: bool,
    pub size: f16,
}

pub struct VoxelMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
}

pub struct VoxelMeshDataWithIndices {
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

impl Voxel {
    #[inline]
    pub fn is_solid(&self) -> bool {
        return self.is_solid;
    }
}
