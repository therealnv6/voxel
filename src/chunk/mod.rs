use bevy::prelude::*;
use noise::OpenSimplex;
use rand::Rng;

use self::{
    event::ChunkCreateEvent,
    events::{draw::ChunkDrawEvent, gen::ChunkGenerateEvent, mesh::ChunkMeshEvent},
    registry::{ChunkRegistry, Coordinates},
};

pub mod chunk;
pub mod discovery;
pub mod event;
pub mod events;
pub mod generation;
pub mod mesh;
pub mod registry;
pub mod voxel;

#[derive(Component)]
pub struct ChunkEntity {
    pub position: Coordinates,
}

#[derive(Resource, Clone)]
pub struct OpenSimplexResource(OpenSimplex);

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

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkRegistry::new())
            .insert_resource(OpenSimplexResource(OpenSimplex::new(
                rand::thread_rng().gen_range(0..=50000),
            )))
            .insert_resource(MeshSettings {
                occlusion_culling: true,
            })
            .insert_resource(DiscoverySettings {
                discovery_radius: 1,
            })
            .insert_resource(GenerationSettings {
                frequency_scale: 0.03,
                amplitude_scale: 20.0,
                threshold: 0.4,
                octaves: 6,
                persistence: 0.5,
            })
            .add_event::<ChunkCreateEvent>()
            .add_event::<ChunkMeshEvent>()
            .add_event::<ChunkGenerateEvent>()
            .add_event::<ChunkDrawEvent>()
            .add_systems(
                Update,
                (
                    event::create_chunk,
                    events::mesh::mesh_chunk,
                    events::mesh::process_chunk_meshing,
                    events::gen::generate_chunk,
                    events::gen::process_chunk_generation,
                    events::draw::draw_chunks,
                    // these are both discovery systems
                    discovery::load_chunks,
                    discovery::unload_distant_chunks,
                ),
            );
    }
}
