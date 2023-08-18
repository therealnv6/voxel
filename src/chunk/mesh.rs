use bevy::{
    prelude::{Mesh, UVec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use super::{
    chunk::VoxelFace,
    voxel::{Voxel, VoxelMeshData},
    MeshSettings,
};

impl Voxel {
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

pub fn mesh(
    voxels: Vec<Voxel>,
    adjacent_voxels: Vec<Vec<Voxel>>,
    settings: MeshSettings,
    UVec3 {
        x: width,
        y: height,
        z: depth,
    }: UVec3,
) -> Mesh {
    // Lists to store vertex positions, colors, and indices for the final mesh
    // relatively ugly, but it works.

    let mut all_vertices = vec![];
    let mut all_colors = vec![];
    let mut all_indices = vec![];

    // Generate voxel mesh data
    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                let index = x + y * width + z * width * height;
                if let Some(voxel) = voxels.get(index as usize) {
                    // not entirely sure why, but `VoxelFace::Back` and `VoxelFace::Top` have to
                    // be the other way around in comparison to the way we declared the indices,
                    // otherwise the wrong sides will be culled.
                    let voxel_faces = [
                        VoxelFace::Back,
                        VoxelFace::Right,
                        VoxelFace::Front,
                        VoxelFace::Left,
                        VoxelFace::Up,
                        VoxelFace::Down,
                    ];

                    let fully_occluded = voxel_faces.iter().all(|face| {
                        get_voxel_face(&voxels, [x, y, z], face.clone(), (width, height, depth))
                            .filter(|voxel| voxel.is_solid())
                            .is_some()
                    });

                    // currently, we're just checking if the voxel is solid. realistically, we
                    // will want to do more checks eventually. things like frustum culling
                    // could perhaps be handled in the same loop (separate function of course).
                    if !voxel.is_solid() || fully_occluded {
                        continue;
                    }

                    // add the voxel size to the dimensions, although voxel size is currently
                    // not actually used and should always be set to 1.0 (refer to the Voxel
                    // struct for more information), we are still applying this here in case we
                    // decide to use the voxel size in the future.
                    let x_pos = x as f32 * voxel.size;
                    let y_pos = y as f32 * voxel.size;
                    let z_pos = z as f32 * voxel.size;

                    let VoxelMeshData { vertices, colors } = voxel.mesh([x_pos, y_pos, z_pos]);

                    // Adjust indices for each voxel
                    let base_vertex_index = all_vertices.len() as u32;

                    // general indices, we're not handling this in the voxel so we can
                    // potentially change up the meshing algorithm sometime to be
                    // greedy meshing, although probably not. will potentially
                    // overcomplicate things in the future in case we add other
                    // functionality (texturing, etc)
                    //
                    // if anyone else reads this (probably not), read more about greedy
                    // meshing here: https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/
                    let indices = [
                        [0, 2, 1, 0, 3, 2],
                        [1, 6, 5, 1, 2, 6],
                        [5, 7, 4, 5, 6, 7],
                        [4, 3, 0, 4, 7, 3],
                        [3, 6, 2, 3, 7, 6],
                        [4, 1, 5, 4, 0, 1],
                    ]
                    .iter()
                    .enumerate()
                    .filter(|(index, _)|
                        // if occlusion culling is disabled, we can
                        // we can simply ignore this.
                        !settings.occlusion_culling
                        // if occlusion culling *should* happen, we will handle this here
                        || check_occluded_with_adjacents(&voxels, &adjacent_voxels, [x, y, z], voxel_faces[*index].clone(), (width, height, depth))
                    )
                    .flat_map(|(_, block)| block)
                    // Add base_vertex_index to each index to match vertex indices;
                    // we have to add this index to handle different locations.
                    .map(|index| index + base_vertex_index)
                    // collect as a Vec<u32>, we have to return a u32 or a u16, and I
                    // decided to opt for a u32. Perhaps we (c/sh)ould move this to a u16?
                    // I'm not entirely sure what the difference is between u16 and u32
                    // indices; is it just the memory usage? I'll do some more
                    // investigation sometime.
                    .collect::<Vec<u32>>();

                    all_indices.extend(indices);
                    all_vertices.extend(vertices);
                    all_colors.extend(colors);
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, all_vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, all_colors);
    mesh.set_indices(Some(Indices::U32(all_indices)));

    // we have to generate the normals for shading; in this case, we'll be using flat normals.
    // should don't see much point in creating our own normal set as they are quite
    // literally.... cubes.
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    mesh
}

pub fn get_voxel_face<'a>(
    voxels: &Vec<Voxel>,
    coordinates: impl Into<UVec3>,
    face: VoxelFace,
    (width, height, _): (u32, u32, u32),
) -> Option<&Voxel> {
    let coordinates = coordinates.into();
    let UVec3 { x, y, z } = coordinates.try_into().unwrap(); // Use UVec3 instead of IVec3

    let (nx, ny, nz) = match face {
        VoxelFace::Front => (x, y, z + 1),
        VoxelFace::Back => (x, y, z - 1),
        VoxelFace::Left => (x - 1, y, z),
        VoxelFace::Right => (x + 1, y, z),
        VoxelFace::Up => (x, y + 1, z),
        VoxelFace::Down => (x, y - 1, z),
    };

    if nx < width && ny < height {
        let index = nx + ny * (width) + nz * (width) * (height);
        voxels.get(index as usize)
    } else {
        None // If the neighboring voxel is outside the bounds, consider it not solid
    }
}
pub fn check_occluded_with_adjacents<'a>(
    voxels: &Vec<Voxel>,
    adjacents: &Vec<Vec<Voxel>>,
    coordinates: impl Into<UVec3>,
    face: VoxelFace,
    (width, height, _): (u32, u32, u32),
) -> bool {
    let coordinates = coordinates.into();
    let UVec3 { x, y, z } = coordinates.try_into().unwrap(); // Use UVec3 instead of IVec3

    let (nx, ny, nz) = match face {
        VoxelFace::Front => (x, y, z + 1),
        VoxelFace::Back => (x, y, z - 1),
        VoxelFace::Left => (x - 1, y, z),
        VoxelFace::Right => (x + 1, y, z),
        VoxelFace::Up => (x, y + 1, z),
        VoxelFace::Down => (x, y - 1, z),
    };

    if nx < width && ny < height {
        let index = nx + ny * (width) + nz * (width) * (height);

        return voxels
            .get(index as usize)
            .filter(|voxel| voxel.is_solid())
            .is_none();
    }

    // Determine which adjacent face to check

    // this logic is flawed. please help.
    let adjacent_chunk = match face {
        VoxelFace::Back => adjacents.get(0),
        VoxelFace::Front => adjacents.get(1),
        VoxelFace::Right => adjacents.get(2),
        VoxelFace::Left => adjacents.get(3),
        VoxelFace::Up => adjacents.get(4),
        VoxelFace::Down => adjacents.get(5),
    };

    if let Some(adj_voxels) = adjacent_chunk {
        // Determine the corresponding index within the adjacent voxels
        let adj_index = match face {
            VoxelFace::Front => (width - 1 - x - nx) + (y + ny) * width + z * width * height,
            VoxelFace::Back => (x + nx) + (y + ny) * width + (height - 1 - z - nz) * width * height,
            VoxelFace::Left => z + nz + (y + ny) * width + (width - 1 - x - nx) * width * height,
            VoxelFace::Right => z + nz + (y + ny) * width + x * width * height,
            VoxelFace::Up => z + nz + y * width + (x + nx) * width * height,
            VoxelFace::Down => z + nz + (width - 1 - y - ny) * width + (x + nx) * width * height,
        };

        if adj_voxels
            .get(adj_index as usize)
            .filter(|voxel| voxel.is_solid())
            .is_some()
        {
            return false;
        }
    }

    true
}
