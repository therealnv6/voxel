use bevy::{
    prelude::{Mesh, UVec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use half::f16;

use super::{
    chunk::VoxelFace,
    voxel::{Voxel, VoxelMeshData},
    MeshSettings,
};

const INDICES_SET: [[u32; 6]; 6] = [
    [0, 2, 1, 0, 3, 2],
    [1, 6, 5, 1, 2, 6],
    [5, 7, 4, 5, 6, 7],
    [4, 3, 0, 4, 7, 3],
    [3, 6, 2, 3, 7, 6],
    [4, 1, 5, 4, 0, 1],
];

pub fn mesh(
    voxels: Vec<Voxel>,
    lod: u32,
    settings: MeshSettings,
    UVec3 {
        x: base_width,
        y: base_height,
        z: base_depth,
    }: UVec3,
) -> Mesh {
    let mut all_vertices = vec![];
    let mut all_colors = vec![];
    let mut all_indices = vec![];

    let lod_multiplier = lod.pow(2);

    let width = base_width >> lod;
    let height = base_height >> lod;
    let depth = base_depth >> lod;

    for z in 0..depth {
        for y in 0..height {
            for x in 0..width {
                let index = (z * base_width * base_height) + (y * base_width) + x;

                if let Some(voxel) = voxels.get(index as usize) {
                    if !voxel.is_solid() {
                        continue;
                    }

                    let voxel_size =
                        f16::from_f32(voxel.size.to_f32() * (lod_multiplier as f32 + 1.0));

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

                    // Adjust indices for each voxel
                    let base_vertex_index = all_vertices.len() as u32;

                    // add the voxel size to the dimensions, although voxel size is currently
                    // not actually used and should always be set to 1.0 (refer to the Voxel
                    // struct for more information), we are still applying this here in case we
                    // decide to use the voxel size in the future.
                    let x_pos = f16::from_f32(x as f32) * voxel_size;
                    let y_pos = f16::from_f32(y as f32) * voxel_size;
                    let z_pos = f16::from_f32(z as f32) * voxel_size;

                    let indices = voxel_faces
                        .into_iter()
                        .enumerate()
                        .filter(|(_, face)| {
                            !settings.occlusion_culling
                                || get_voxel_face(
                                    &voxels,
                                    [x, y, z],
                                    &face,
                                    (base_width, base_height, base_depth),
                                )
                                .is_none()
                        })
                        .map(|(index, _)| {
                            INDICES_SET[index]
                                .iter()
                                .map(|index| index + base_vertex_index)
                                .collect::<Vec<u32>>()
                        })
                        .flatten();

                    let VoxelMeshData { vertices, colors } =
                        voxel.mesh([x_pos, y_pos, z_pos], voxel_size);

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

    // do we need aabb (axis aligned bounding boxes)? i feel like it would help with GPU frustum
    // culling, and perhaps other GPU culling.
    mesh.compute_aabb();

    mesh
}

pub fn get_voxel_face<'a>(
    voxels: &'a Vec<Voxel>,
    coordinates: impl Into<UVec3>,
    face: &'a VoxelFace,
    (width, height, _): (u32, u32, u32),
) -> Option<&'a Voxel> {
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
        return voxels
            .get((nx + ny * (width) + nz * (width) * (height)) as usize)
            .filter(|voxel| voxel.is_solid());
    }

    None
}
