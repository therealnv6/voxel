use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use noise::{NoiseFn, OpenSimplex};
use rand::Rng;

use crate::chunk::{registry::ChunkRegistry, voxel::Voxel, DiscoverySettings, MeshSettings};

const FREQUENCY_SCALE: f64 = 0.1;
const AMPLITUDE_SCALE: f64 = 5.0;
const THRESHOLD: f64 = 0.0;

const OCTAVES: i32 = 4;
const PERSISTENCE: f64 = 0.5;

#[derive(Component)]
pub struct ComputeGen(Task<()>);

fn generate_color_from_heat(heat: f64) -> Color {
    const DARK_FACTOR: f64 = 0.5;
    const SENSITIVITY: f64 = 3.0;

    let modified_heat = (heat * SENSITIVITY).max(0.0);

    let r = (1.0 - modified_heat).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let g = modified_heat.sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let b = (modified_heat - 1.0).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;

    Color::rgb(r as f32, g as f32, b as f32)
}

pub fn process_generation(mut commands: Commands) {
    let queue = super::get_generation_queue();
    let entries = &queue.queue;

    if entries.is_empty() {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    for (chunk, _) in entries {
        let chunk = chunk.clone();

        let task: Task<()> = thread_pool.spawn(async move {
            if let Ok(mut chunk) = chunk.lock() {
                if chunk.is_generated() {
                    return;
                }

                let (width, height, depth) = chunk.get_dimensions();
                let simplex = OpenSimplex::new(rand::thread_rng().gen_range(0..50000));

                for x in 0..width {
                    for z in 0..depth {
                        for y in 0..height {
                            let mut amplitude = 1.0;

                            let noise_value = (0..OCTAVES)
                                .map(|_| {
                                    let p = [
                                        x as f64 * FREQUENCY_SCALE,
                                        y as f64 * FREQUENCY_SCALE,
                                        z as f64 * FREQUENCY_SCALE,
                                    ];

                                    let value = simplex.get(p);
                                    let result = value * amplitude;

                                    amplitude *= PERSISTENCE;

                                    result
                                })
                                .sum::<f64>()
                                * AMPLITUDE_SCALE;

                            let is_present = noise_value > THRESHOLD;

                            if is_present {
                                let heat = ((noise_value - THRESHOLD)
                                    / (AMPLITUDE_SCALE - THRESHOLD))
                                    .max(0.0)
                                    .min(1.0);

                                let color = generate_color_from_heat(heat);

                                let voxel = Voxel {
                                    color,
                                    size: 1.0,
                                    is_solid: true,
                                };

                                chunk.set_voxel([x, y, z], voxel);
                            }
                        }
                    }
                }

                chunk.set_generated(true);
            }
        });

        commands.spawn(ComputeGen(task));
    }
}

pub fn handle_gen_tasks(mut commands: Commands, mut tasks: Query<(Entity, &mut ComputeGen)>) {
    for (entity, mut task) in &mut tasks {
        if let Some(()) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<ComputeGen>();
        }
    }
}
