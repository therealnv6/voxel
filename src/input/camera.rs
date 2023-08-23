use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::EventReader};

pub const DEFAULT_CAMERA_SENS: f32 = 0.005;

#[derive(Default, Component)]
pub struct PlayerController {
    yaw: f32,
    pitch: f32,
    pub locked: bool,
}

pub fn handle_mouse(
    mut query: Query<(&mut PlayerController, &mut Transform)>,
    mut reader: EventReader<MouseMotion>,
) {
    let (mut controller, mut transform) = query.single_mut();
    let mut delta = Vec2::ZERO;

    if controller.locked {
        for mouse_move in reader.iter() {
            delta += mouse_move.delta;
        }
    }

    if delta == Vec2::ZERO {
        return;
    }

    let mut new_pitch = delta.y.mul_add(DEFAULT_CAMERA_SENS, controller.pitch);
    let new_yaw = delta.x.mul_add(-DEFAULT_CAMERA_SENS, controller.yaw);

    new_pitch = new_pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

    controller.yaw = new_yaw;
    controller.pitch = new_pitch;

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, new_yaw) * Quat::from_axis_angle(-Vec3::X, new_pitch);
}

pub fn handle_move(
    mut query: Query<&mut Transform, With<PlayerController>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    let forward = transform.forward();
    let right = transform.right();

    let mut acceleration = 0.05f32;

    {
        let movement_bindings = [
            (KeyCode::W, Vec3::new(0.0, 0.0, 1.0)),
            (KeyCode::S, Vec3::new(0.0, 0.0, -1.0)),
            (KeyCode::D, Vec3::new(1.0, 0.0, 0.0)),
            (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0)),
            (KeyCode::Space, Vec3::new(0.0, 1.0, 0.0)),
            (KeyCode::ShiftLeft, Vec3::new(0.0, -1.0, 0.0)),
        ];

        for (keycode, dir) in movement_bindings.into_iter() {
            if keys.pressed(keycode) {
                direction += dir;
            }
        }
    }

    if keys.pressed(KeyCode::ControlLeft) {
        acceleration *= 8.0;
    }

    if direction != Vec3::ZERO {
        transform.translation += direction.x * right * acceleration
            + direction.z * forward * acceleration
            + direction.y * Vec3::Y * acceleration;
    }
}
