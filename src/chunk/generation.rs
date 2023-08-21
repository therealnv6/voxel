use bevy::prelude::*;
use rayon::prelude::*;

use crate::chunk::{voxel::Voxel, GenerationSettings};
use half::f16;
use noise::{NoiseFn, OpenSimplex};

pub fn generate_voxels(
    settings: GenerationSettings,
    simplex: OpenSimplex,
    IVec3 {
        x: world_pos_x,
        y: world_pos_y,
        z: world_pos_z,
    }: IVec3,
    (width, height, depth): (u32, u32, u32),
) -> Vec<Voxel> {
    let mut voxels: Vec<Voxel> = vec![
        Voxel {
            size: f16::from_f32(1.0),
            is_solid: false,
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
        };
        (width * height * depth).try_into().unwrap()
    ];

    let frequency_scale: f64 = settings.frequency_scale;
    let amplitude_scale: f64 = settings.amplitude_scale;
    let threshold: f64 = settings.threshold;

    let octaves: i32 = settings.octaves;
    let persistence: f64 = settings.persistence;

    let amplitudes: Vec<f64> = (0..octaves).map(|i| persistence.powi(i)).collect(); // Precompute amplitudes

    let width_scale = frequency_scale / width as f64;
    let height_scale = frequency_scale / height as f64;

    voxels
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, voxel)| {
            let z = index / (width * height) as usize;
            let y = (index % (width * height) as usize) / width as usize;
            let x = index % width as usize;

            let z_coord = (z as f64 + world_pos_z as f64) * frequency_scale;
            let z_coord_with_offset = z_coord + (z as f64 / depth as f64) * width_scale;

            let x_coord = (x as f64 + world_pos_x as f64) * frequency_scale;
            let x_coord_with_offset = x_coord + (x as f64 / width as f64) * width_scale;

            let y_coord = (y as f64 + world_pos_y as f64) * frequency_scale;
            let y_coord_with_offset = y_coord + (y as f64 / height as f64) * height_scale;

            let mut noise_value = 0.0;
            for i in 0..octaves {
                let p = [
                    x_coord_with_offset,
                    y_coord_with_offset,
                    z_coord_with_offset,
                    (i as f64) * 10.0,
                ];
                noise_value += amplitudes[i as usize] * simplex.get(p);
            }

            noise_value *= amplitude_scale;
            if noise_value > threshold {
                let heat = ((noise_value - threshold) / (amplitude_scale - threshold))
                    .max(0.0)
                    .min(1.0);

                let color = generate_color_from_heat(heat);

                *voxel = Voxel {
                    color,
                    size: f16::from_f32(1.0),
                    is_solid: true,
                };
            }
        });

    voxels
}
#[inline]
fn generate_color_from_heat(heat: f64) -> Color {
    const DARK_FACTOR: f64 = 0.3;
    const SENSITIVITY: f64 = 5.0;

    let modified_heat = (heat * SENSITIVITY).max(0.0);

    let r = (1.0 - modified_heat).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let g = modified_heat.sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;
    let b = (modified_heat - 1.0).sqrt() * (1.0 - DARK_FACTOR) + DARK_FACTOR;

    Color::rgb(r as f32, g as f32, b as f32)
}
