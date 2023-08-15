use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use noise::{NoiseFn, OpenSimplex};
use rand::Rng;

use crate::chunk::{voxel::Voxel, GenerationSettings};

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

pub fn process_generation(mut commands: Commands, settings: Res<GenerationSettings>) {
    let queue = super::get_generation_queue();
    let entries = &queue.queue;

    if entries.is_empty() {
        return;
    }

    let frequency_scale: f64 = settings.frequency_scale;
    let amplitude_scale: f64 = settings.amplitude_scale;
    let threshold: f64 = settings.threshold;

    let octaves: i32 = settings.octaves;
    let persistence: f64 = settings.persistence;

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

                            let noise_value = (0..octaves)
                                .map(|_| {
                                    let p = [
                                        x as f64 * frequency_scale,
                                        y as f64 * frequency_scale,
                                        z as f64 * frequency_scale,
                                    ];

                                    let value = simplex.get(p);
                                    let result = value * amplitude;

                                    amplitude *= persistence;

                                    result
                                })
                                .sum::<f64>()
                                * amplitude_scale;

                            let is_present = noise_value > threshold;

                            if is_present {
                                let heat = ((noise_value - threshold)
                                    / (amplitude_scale - threshold))
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
