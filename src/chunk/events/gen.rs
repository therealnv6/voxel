use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

use crate::chunk::{
    generation::generate_voxels,
    registry::{ChunkRegistry, Coordinates},
    voxel::Voxel,
    GenerationSettings, OpenSimplexResource,
};

#[derive(Event)]
pub struct ChunkGenerateEvent {
    pub coordinates: Coordinates,
}

#[derive(Component)]
pub struct ChunkGenerationTask(Task<(Coordinates, Vec<Voxel>)>);

pub fn generate_chunk(
    mut commands: Commands,
    mut reader: EventReader<ChunkGenerateEvent>,
    mut registry: ResMut<ChunkRegistry>,
    settings: Res<GenerationSettings>,
    simplex: Res<OpenSimplexResource>,
) {
    let pool = AsyncComputeTaskPool::get();

    for ChunkGenerateEvent { coordinates } in reader.iter() {
        let coordinates = *coordinates;
        let Some(chunk) = registry.get_chunk_at_mut(coordinates) else {
            continue;
        };

        let settings = settings.clone();
        let simplex = simplex.0;

        let world_position = chunk.world_position;

        let task = pool.spawn(async move {
            let voxels = generate_voxels(
                &settings,
                simplex,
                world_position,
                (
                    ChunkRegistry::CHUNK_SIZE as u32,
                    ChunkRegistry::CHUNK_HEIGHT as u32,
                    ChunkRegistry::CHUNK_SIZE as u32,
                ),
            );

            return (coordinates, voxels);
        });

        commands.spawn(ChunkGenerationTask(task));
    }
}

pub fn process_chunk_generation(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ChunkGenerationTask)>,
    mut registry: ResMut<ChunkRegistry>,
) {
    tasks.iter_mut().for_each(|(entity, mut task)| {
        let task = &mut task.0;
        let Some((coordinates, voxels)) = future::block_on(future::poll_once(task)) else {
            return;
        };

        commands.entity(entity).remove::<ChunkGenerationTask>();

        let Some(chunk) = registry.get_chunk_at_mut(coordinates) else {
            return;
        };

        chunk.set_voxels(voxels);
        chunk.set_busy(false);
        chunk.set_dirty(true);
        chunk.set_generated(true);
    });
}
