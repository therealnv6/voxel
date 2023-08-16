use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use super::{
    chunk::{Chunk, VoxelFace},
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

impl Chunk {
    /// Generates a mesh for the chunk's solid voxels and returns a handle to the created mesh.
    ///
    /// This method generates a mesh for the solid voxels present in the chunk. It iterates over
    /// each voxel in the chunk and creates mesh data for the solid voxels. The generated mesh
    /// contains attributes for vertex positions and colors, as well as indices for rendering
    /// triangles.
    ///
    /// # Parameters
    ///
    /// - `meshes`: A mutable reference to the `Assets<Mesh>` resource used for managing meshes.
    ///
    /// # Returns
    ///
    /// A handle to the generated mesh, which can be used for rendering.
    pub fn mesh(&mut self, settings: MeshSettings) -> Mesh {
        // Lists to store vertex positions, colors, and indices for the final mesh
        // relatively ugly, but it works.
        let mut all_vertices = vec![];
        let mut all_colors = vec![];
        let mut all_indices = vec![];

        // Generate voxel mesh data
        for x in 0..self.width {
            for y in 0..self.height {
                for z in 0..self.depth {
                    if let Some(voxel) = self.get_voxel([x, y, z]) {
                        // currently, we're just checking if the voxel is solid. realistically, we
                        // will want to do more checks eventually. things like frustum culling
                        // could perhaps be handled in the same loop (separate function of course).
                        if !voxel.is_solid() {
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
                        let mut indices = [
                            0, 2, 1, 0, 3, 2, // Front face
                            1, 6, 5, 1, 2, 6, // Right face
                            5, 7, 4, 5, 6, 7, // Back face
                            4, 3, 0, 4, 7, 3, // Left face
                            3, 6, 2, 3, 7, 6, // Top face
                            4, 1, 5, 4, 0, 1, // Bottom face
                        ]
                        .iter()
                        // Add base_vertex_index to each index to match vertex indices;
                        // we have to add this index to handle different locations.
                        .map(|index| index + base_vertex_index)
                        // collect as a Vec<u32>, we have to return a u32 or a u16, and I
                        // decided to opt for a u32. Perhaps we (c/sh)ould move this to a u16?
                        // I'm not entirely sure what the difference is between u16 and u32
                        // indices; is it just the memory usage? I'll do some more
                        // investigation sometime.
                        .collect::<Vec<u32>>();

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

                        if settings.occlusion_culling {
                            let face_index_count = 6; // number of indices per face
                            voxel_faces
                                .iter()
                                // we need the index for getting the correct index for removing the
                                // indices, so we're enumerating over the faces.
                                .enumerate()
                                .filter_map(|(index, face)| {
                                    // get the neighboring voxel depending on the face
                                    self.get_voxel_face([x, y, z], face.clone())
                                        // i genuinely didn't know you could call .filter() on an
                                        // Option<T>, but well, this works.
                                        .filter(|voxel| voxel.is_solid())
                                        .map(|_| {
                                            index * face_index_count..(index + 1) * face_index_count
                                        })
                                })
                                // flatten from the Option<T>, as we only need the ones with an actual
                                // value. the other ones can simply just be ignored.
                                .flatten()
                                // reverse the order to avoid shifting errors, as removing from the
                                // start of the vector will simply shift the other indices down, thus
                                // the indices being incorrect.
                                .rev()
                                // actually remove the indices; these are the indices that are occluded
                                // thus should be removed from the indices before they are added to the
                                // all_indices variable.
                                .for_each(|idx| {
                                    indices.remove(idx);
                                });
                        }

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
}
