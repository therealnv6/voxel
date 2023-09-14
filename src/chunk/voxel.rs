use bevy::prelude::Color;
use half::f16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voxel {
    pub color: Color,
    pub is_solid: bool,
    pub size: f16,
}

pub struct VoxelMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
}

impl Voxel {
    pub fn new(color: Color, is_solid: bool, size: f16) -> Self {
        Self {
            color,
            is_solid,
            size,
        }
    }

    pub fn new_solid(color: Color, size: f16) -> Self {
        Self {
            color,
            is_solid: true,
            size,
        }
    }

    #[inline]
    pub fn is_solid(&self) -> bool {
        return self.is_solid;
    }

    pub fn mesh(&self, [x, y, z]: [f16; 3], size: f16) -> VoxelMeshData {
        let [x, y, z] = [x.to_f32(), y.to_f32(), z.to_f32()];
        let size = size.to_f32();

        VoxelMeshData {
            vertices: vec![
                [x, y, z],
                [x + size, y, z],
                [x + size, y + size, z],
                [x, y + size, z],
                [x, y, z + size],
                [x + size, y, z + size],
                [x + size, y + size, z + size],
                [x, y + size, z + size],
            ],
            // the colors are repeated 8 times to cover the entire cube. there are 24 vertices, which
            // is 8 (24/3 = 8, 3 is x,y,z). we have to cover all of those to cover the entirity of the
            // cube, otherwise we will be having a mismatched amount of attributes. this also allows
            // for a gradient effect on a single voxel, but i see no point in implementing this. could
            // always be something cool for in the future.
            colors: vec![self.color.into(); 8],
        }
    }
}

impl Default for Voxel {
    fn default() -> Self {
        Self {
            size: f16::from_f32(1.0),
            is_solid: false,
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
        }
    }
}
