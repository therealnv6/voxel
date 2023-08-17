use crate::chunk::{registry::Coordinates, voxel::Voxel, GenerationSettings};
use bevy::prelude::*;

use noise::{NoiseFn, OpenSimplex};

pub fn generate_voxels(
    settings: GenerationSettings,
    simplex: OpenSimplex,
    Coordinates(world_pos_x, world_pos_z): Coordinates,
    (width, height, depth): (u32, u32, u32),
) -> Vec<Voxel> {
    let mut voxels: Vec<Voxel> = Vec::new();

    voxels.resize(
        (width * height * depth)
            .try_into()
            .expect("width*height*depth could not be mapped into usize, how big are your chunks?"),
        Voxel {
            size: 1.0,
            is_solid: false,
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
        },
    );

    let frequency_scale: f64 = settings.frequency_scale;
    let amplitude_scale: f64 = settings.amplitude_scale;
    let threshold: f64 = settings.threshold;

    let octaves: i32 = settings.octaves;
    let persistence: f64 = settings.persistence;

    let mut amplitudes = vec![1.0; octaves.try_into().unwrap()]; // Precompute amplitudes

    let width_scale = frequency_scale / width as f64;
    let depth_scale = frequency_scale / depth as f64;

    for i in 1..octaves {
        amplitudes.insert(i as usize, amplitudes[(i - 1) as usize] * persistence);
    }

    for z in 0..depth {
        let z_coord = (z as f64 + world_pos_z as f64) * frequency_scale;

        let z_coord_with_offset = z_coord + (z as f64 / depth as f64) * depth_scale;

        for x in 0..width {
            let x_coord = (x as f64 + world_pos_x as f64) * frequency_scale;

            let x_coord_with_offset = x_coord + (x as f64 / width as f64) * width_scale;

            for y in 0..height {
                let y_coord = y as f64 * frequency_scale;

                let mut noise_value = 0.0;
                for i in 0..octaves {
                    let p = [
                        x_coord_with_offset,
                        y_coord,
                        z_coord_with_offset,
                        (i as f64) * 10.0, // Add an offset to create variation in noise patterns
                    ];

                    noise_value += amplitudes[i as usize] * simplex.get(p);
                }

                noise_value *= amplitude_scale;
                if noise_value > threshold {
                    let heat = ((noise_value - threshold) / (amplitude_scale - threshold))
                        .max(0.0)
                        .min(1.0);

                    let color = generate_color_from_heat(heat);
                    let index = x + y * width + z * width * height;

                    voxels[index as usize] = Voxel {
                        color,
                        size: 1.0,
                        is_solid: true,
                    };
                }
            }
        }
    }

    return voxels;
}

#[inline]
fn generate_color_from_heat(heat: f64) -> Color {
    const DARK_FACTOR: f64 = 0.3;
    const SENSITIVITY: f64 = 15.0;

    let modified_heat = (heat * SENSITIVITY).max(0.0);

    let r = (1.0 - modified_heat).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let g = modified_heat.sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let b = (modified_heat - 1.0).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;

    Color::rgb(r as f32, g as f32, b as f32)
}
