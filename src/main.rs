use bevy::{core_pipeline::fxaa::Fxaa, prelude::*, window::PresentMode};
use input::{camera::PlayerController, InputPlugin};

pub mod chunk;
pub mod input;
pub mod ui;
pub mod util;
pub mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("bevy voxels"),
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            chunk::ChunkPlugin,
            // this plugin is currently disabled due to performance reasons.
            // world::WorldPlugin,
            InputPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: (90.0 / 360.0) * (std::f32::consts::PI * 2.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        Fxaa::default(),
        PlayerController::default(),
    ));
}
