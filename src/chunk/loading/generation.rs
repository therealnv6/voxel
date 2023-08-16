use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use noise::{NoiseFn, OpenSimplex};
use rand::Rng;

use crate::chunk::{voxel::Voxel, GenerationSettings, OpenSimplexResource};

#[derive(Component)]
pub struct ComputeGen(Task<()>);

fn generate_color_from_heat(heat: f64) -> Color {
    const DARK_FACTOR: f64 = 0.3;
    const SENSITIVITY: f64 = 5000.0;

    let modified_heat = (heat * SENSITIVITY).max(0.0);

    let r = (1.0 - modified_heat).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let g = modified_heat.sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let b = (modified_heat - 1.0).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;

    Color::rgb(r as f32, g as f32, b as f32)
}

pub fn process_generation(
    mut commands: Commands,
    settings: Res<GenerationSettings>,
    simplex: Res<OpenSimplexResource>,
) {
    let mut queue = super::get_generation_queue();
    let entries = &mut queue.queue;

    if entries.is_empty() {
        return;
    }

    let simplex = simplex.0;

    let frequency_scale: f64 = settings.frequency_scale;
    let amplitude_scale: f64 = settings.amplitude_scale;
    let threshold: f64 = settings.threshold;

    let octaves: i32 = settings.octaves;
    let persistence: f64 = settings.persistence;

    let thread_pool = AsyncComputeTaskPool::get();

    // we want to drain here instead of just looping over all of them, otherwise we will get
    // gigantic spikes because of handling every generation task in the same frame.
    for (chunk, _) in entries.drain(0..entries.len().min(4)) {
        let chunk = chunk.clone();

        let task: Task<()> = thread_pool.spawn(async move {
            if let Ok(mut chunk) = chunk.lock() {
                if chunk.is_generated() {
                    return;
                }

                let (width, height, depth) = chunk.get_dimensions();

                for x in 0..width {
                    for z in 0..depth {
                        for y in 0..height {
                            let mut amplitude = 1.0;

                            let noise_value = (0..octaves)
                                .map(|_| {
                                    let x_coord = (x as f64 * frequency_scale)
                                        + (((chunk.world_position.0 * 2) / width as i32) as f64
                                            + 1.0);
                                    let y_coord = y as f64 * frequency_scale;
                                    let z_coord = (z as f64 * frequency_scale)
                                        + (((chunk.world_position.1 * 2) / depth as i32) as f64
                                            + 1.0);

                                    let p = [x_coord, y_coord, z_coord];

                                    let value = simplex.get(p);
                                    let result = value * amplitude;

                                    amplitude *= persistence;

                                    result
                                })
                                .sum::<f64>()
                                * amplitude_scale;

                            if noise_value > threshold {
                                let heat = ((noise_value - threshold)
                                    / (amplitude_scale - threshold))
                                    .max(0.0)
                                    .min(1.0);

                                let color = generate_color_from_heat(heat);

                                chunk.set_voxel(
                                    [x, y, z],
                                    Voxel {
                                        color,
                                        size: 1.0,
                                        is_solid: true,
                                    },
                                );
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
