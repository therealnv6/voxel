use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use self::{
    loading::{ChunkDrawingQueue, ChunkMeshingQueue},
    registry::ChunkRegistry,
};

pub mod chunk;
pub mod loading;
pub mod mesh;
pub mod registry;
pub mod voxel;

#[derive(Resource, Clone)]
pub struct MeshSettings {
    pub occlusion_culling: bool,
}

#[derive(Resource, Clone)]
pub struct DiscoverySettings {
    // we don't need much more than an u8 for the discovery radius.
    pub discovery_radius: i8,
}

#[derive(Resource, Clone)]
pub struct GenerationSettings {
    pub frequency_scale: f64,
    pub amplitude_scale: f64,
    pub threshold: f64,
    pub octaves: i32,
    pub persistence: f64,
}

pub struct ChunkPlugin;

const DRAW_DELAY_MILLIS: u64 = 20;
const DISCOVERY_DELAY_MILLIS: u64 = 500;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkRegistry::new());
        app.insert_resource(ChunkMeshingQueue::default());
        app.insert_resource(ChunkDrawingQueue::default());

        app.insert_resource(MeshSettings {
            occlusion_culling: true,
        });

        app.insert_resource(DiscoverySettings {
            discovery_radius: 5,
        });

        app.insert_resource(GenerationSettings {
            frequency_scale: 0.1,
            amplitude_scale: 5.0,
            threshold: 0.0,
            octaves: 4,
            persistence: 0.5,
        });

        let delay = Duration::from_millis(DISCOVERY_DELAY_MILLIS);

        app.add_systems(
            Update,
            (
                // the following systems are only executed every few milliseconds, because they
                // actively lock objects to be able to access them from other threads. it shouldn't
                // be too big of a difference in visual representation as long as we don't change
                // the delay to be something significantly higher.
                loading::discovery::load_chunks.run_if(on_timer(delay)),
                loading::discovery::handle_mesh_tasks.run_if(on_timer(delay)),
                // this doesn't matter *too* much if it's ran often, thus a different delay than
                // the 2 systems above. we'll be tweaking this sometime
                loading::draw::draw_chunks
                    .run_if(on_timer(Duration::from_millis(DRAW_DELAY_MILLIS))),
                loading::unload::unload_distant_chunks
                    .run_if(on_timer(Duration::from_millis(DRAW_DELAY_MILLIS))),
                // these are chunk generation systems; they are relatively resource-intensive, thus
                // they are slower than the 2 systems above. we might want to tweak these in
                // the end as well.
                loading::generation::process_generation.run_if(on_timer(delay)),
                loading::generation::handle_gen_tasks.run_if(on_timer(delay)),
            ),
        );
    }
}
