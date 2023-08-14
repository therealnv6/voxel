use super::voxel::Voxel;

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub voxels: Vec<Voxel>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub dirty: bool,
}

impl Chunk {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let num_voxels = width * height * depth;
        let voxels = vec![Voxel::default(); num_voxels];

        Chunk {
            voxels,
            width,
            height,
            depth,
            dirty: false,
        }
    }

    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> Option<&Voxel> {
        if x < self.width && y < self.height && z < self.depth {
            let index = self.get_index(x, y, z);
            Some(&self.voxels[index])
        } else {
            None
        }
    }

    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, voxel: Voxel) {
        if x < self.width && y < self.height && z < self.depth {
            let index = self.get_index(x, y, z);
            self.voxels[index] = voxel;
        }
    }

    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.width + z * self.width * self.height
    }
}
