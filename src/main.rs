use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use chunk::{
    registry::{ChunkRegistry, Coordinates},
    voxel::Voxel,
};
use input::InputPlugin;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

pub mod chunk;
pub mod input;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            WireframePlugin,
            chunk::ChunkPlugin,
            LookTransformPlugin,
            FpsCameraPlugin::default(),
            WorldInspectorPlugin::new(),
            InputPlugin,
        ))
        .add_systems(Startup, (setup, add_mesh))
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                // no smoothing, we're just using this plugin because... well.. i'm lazy.
                smoothing_weight: 0.0,
                ..default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

pub fn add_mesh(mut commands: Commands, meshes: ResMut<Assets<Mesh>>) {
    let mut registry = ChunkRegistry::new();
    let chunk = registry.get_chunk_at(Coordinates(0, 0));

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let color = Color::from([x as f32 / 15.0, y as f32 / 15.0, z as f32 / 15.0, 1.0]);

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

    let mesh = chunk.mesh(meshes);

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone().into(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::splat(0.2)),
            ..default()
        },
        Wireframe,
    ));
}
