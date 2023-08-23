use bevy::{prelude::*, window::PresentMode};

use input::InputPlugin;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

pub mod chunk;
pub mod input;
pub mod ui;
pub mod util;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("bevy voxels"),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            chunk::ChunkPlugin,
            LookTransformPlugin,
            FpsCameraPlugin::default(),
            InputPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: (90.0 / 360.0) * (std::f32::consts::PI * 2.0),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                smoothing_weight: 0.1,
                mouse_rotate_sensitivity: Vec2::splat(0.9),
                translate_sensitivity: 35.0,
                ..default()
            },
            Vec3::new(100.0, 120.0, 100.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}
