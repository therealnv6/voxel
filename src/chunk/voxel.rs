use bevy::prelude::Color;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Voxel {
    color: Color,
    size: f32,
}
