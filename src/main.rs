use bevy::prelude::*;

pub mod chunk;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, chunk::ChunkPlugin))
        .run();
}
