use std::time::Duration;

use crate::chunk::{
    registry::{ChunkRegistry, Coordinates},
    ChunkEntity,
};

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};

#[derive(Event)]
pub struct ChunkDrawEvent {
    pub coordinates: Coordinates,
}

pub fn draw_chunks(
    mut commands: Commands,
    mut reader: EventReader<ChunkDrawEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_cache: Local<Option<Handle<StandardMaterial>>>,
    mut registry: ResMut<ChunkRegistry>,
) {
    let material = material_cache.get_or_insert_with(|| materials.add(StandardMaterial::default()));
    for ChunkDrawEvent {
        coordinates: Coordinates { x, y, z },
    } in reader.iter()
    {
        let (x, y, z) = (*x, *y, *z);

        let Some(chunk) = registry.get_chunk_at_mut([x, y, z]) else {
            continue;
        };

        if let Some(mesh) = chunk.get_mesh() {
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(550),
                TransformPositionLens {
                    start: Vec3::new(x as f32, y as f32 - 12.0, z as f32),
                    end: Vec3::new(x as f32, y as f32, z as f32),
                },
            );

            let bundle = PbrBundle {
                mesh,
                material: material.clone(),
                ..Default::default()
            };

            chunk.set_drawn(true);
            commands
                .spawn(bundle)
                .insert(ChunkEntity {
                    position: (x, y, z).into(),
                })
                .insert(Animator::new(tween));
        }
    }
}
