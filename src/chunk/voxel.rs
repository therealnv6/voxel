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
    pub fn mesh(&self, [x, y, z]: [f32; 3]) -> VoxelMeshData {
        let voxel_size = self.size;

        let x_pos = x * voxel_size;
        let y_pos = y * voxel_size;
        let z_pos = z * voxel_size;

        // the colors are repeated 8 times to cover the entire cube. there are 24 vertices, which
        // is 8 (24/3 = 8, 3 is x,y,z). we have to cover all of those to cover the entirity of the
        // cube, otherwise we will be having a mismatched amount of attributes. this also allows
        // for a gradient effect on a single voxel, but i see no point in implementing this. could
        // always be something cool for in the future.
        let colors = vec![self.color.into(); 8];

        // these are just the vertices of a cube, using voxel_size to change the size of the cube.
        // although the voxel_size variable in the Voxel struct is currently deprecated and should
        // always be 1.0 (refer to the Voxel struct for further information), we're still applying
        // them here just in case we decide to do anything with the voxel size in the future.
        let vertices = vec![
            [x_pos, y_pos, z_pos],
            [x_pos + voxel_size, y_pos, z_pos],
            [x_pos + voxel_size, y_pos + voxel_size, z_pos],
            [x_pos, y_pos + voxel_size, z_pos],
            [x_pos, y_pos, z_pos + voxel_size],
            [x_pos + voxel_size, y_pos, z_pos + voxel_size],
            [x_pos + voxel_size, y_pos + voxel_size, z_pos + voxel_size],
            [x_pos, y_pos + voxel_size, z_pos + voxel_size],
        ];

        VoxelMeshData { vertices, colors }
    }
}
