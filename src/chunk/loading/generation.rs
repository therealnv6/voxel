use crate::chunk::{voxel::Voxel, GenerationSettings, OpenSimplexResource};
use bevy::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use noise::NoiseFn;

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

    for (chunk, _) in entries.drain(0..entries.len().min(4)) {
        let chunk = chunk.clone();

        let task: Task<()> = thread_pool.spawn(async move {
            if let Ok(mut chunk) = chunk.lock() {
                if chunk.is_generated() {
                    return;
                }

                let (width, height, depth) = chunk.get_dimensions();

                for z in 0..depth {
                    for x in 0..width {
                        for y in 0..height {
                            let mut amplitude = 1.0;

                            let x_coord =
                                (x as f64 + chunk.world_position.0 as f64) * frequency_scale;
                            let y_coord = y as f64 * frequency_scale;
                            let z_coord =
                                (z as f64 + chunk.world_position.1 as f64) * frequency_scale;

                            let noise_value = (0..octaves)
                                .map(|_| {
                                    let x_coord_with_offset =
                                        x_coord + (x as f64 / width as f64) * frequency_scale;
                                    let z_coord_with_offset =
                                        z_coord + (z as f64 / depth as f64) * frequency_scale;

                                    let p = [x_coord_with_offset, y_coord, z_coord_with_offset];

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

pub fn handle_gen_tasks(mut commands: Commands, tasks: Query<(Entity, &ComputeGen)>) {
    tasks
        .iter()
        .take(2)
        .filter(|(_, ComputeGen(task))| task.is_finished())
        .for_each(|(entity, _)| {
            commands.entity(entity).remove::<ComputeGen>();
        });
}
