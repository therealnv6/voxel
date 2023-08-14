use bevy::prelude::*;

pub mod chunk;
pub mod linearization;
pub mod registry;
pub mod voxel;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        println!("added chunk plugin!");
    }
}
