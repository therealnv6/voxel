use std::time::Duration;

use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, time::common_conditions::on_timer,
};
use noise::OpenSimplex;
use rand::Rng;

use self::{
    event::ChunkCreateEvent,
    events::{
        discovery::ChunkDiscoveryEvent, draw::ChunkDrawEvent, gen::ChunkGenerateEvent,
        mesh::ChunkMeshEvent,
    },
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
    pub discovery_radius: i8,
    pub discovery_radius_height: i8,
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
                discovery_radius: 4,
                discovery_radius_height: 2,
            })
            .insert_resource(GenerationSettings {
                frequency_scale: 0.03,
                amplitude_scale: 20.0,
                threshold: 0.4,
                octaves: 2,
                persistence: 0.5,
            })
            .add_event::<ChunkCreateEvent>()
            .add_event::<ChunkMeshEvent>()
            .add_event::<ChunkDiscoveryEvent>()
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
                    events::discovery::handle_chunk_discovery
                        .run_if(input_toggle_active(true, KeyCode::L)),
                    events::discovery::process_discovery_tasks,
                    discovery::unload_distant_chunks
                        .run_if(on_timer(Duration::from_millis(500)))
                        .run_if(input_toggle_active(true, KeyCode::M)),
                ),
            );
    }
}
