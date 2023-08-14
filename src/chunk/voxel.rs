use bevy::prelude::Color;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Voxel {
    pub color: Color,
    pub is_solid: bool,
    pub size: f32,
}

pub struct VoxelMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
}

impl Voxel {
    pub fn is_solid(&self) -> bool {
        // we'll want to change this sometime
        return self.is_solid;
    }
}
