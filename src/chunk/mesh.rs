use bevy::{
    prelude::{Assets, Handle, Mesh, ResMut},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use super::{chunk::Chunk, voxel::VoxelMeshData};

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
    pub fn mesh(&mut self, mut meshes: ResMut<Assets<Mesh>>) -> Handle<Mesh> {
        // Lists to store vertex positions, colors, and indices for the final mesh
        // relatively ugly, but it works.
        let mut all_vertices = vec![];
        let mut all_colors = vec![];
        let mut all_indices = vec![];

        // Generate voxel mesh data
        for x in 0..self.width {
            for y in 0..self.height {
                for z in 0..self.depth {
                    if let Some(voxel) = self.get_voxel(x, y, z) {
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

                        all_indices.extend(
                            // general indices, we're not handling this in the voxel so we can
                            // potentially change up the meshing algorithm sometime to be
                            // greedy meshing, although probably not. will potentially
                            // overcomplicate things in the future in case we add other
                            // functionality (texturing, etc)
                            //
                            // if anyone else reads this (probably not), read more about greedy
                            // meshing here: https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/
                            [
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
                            .collect::<Vec<u32>>(),
                        );

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

        // Update the mesh_id field with the new mesh
        self.mesh_id = Some(meshes.add(mesh));

        // Return the handle to the mesh
        self.mesh_id.clone().unwrap()
    }
}
