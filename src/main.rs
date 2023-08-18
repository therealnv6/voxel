use bevy_tweening::TweeningPlugin;
use std::f32::consts::PI;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active,
    pbr::{wireframe::WireframePlugin, CascadeShadowConfigBuilder},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
    window::PresentMode,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, DefaultInspectorConfigPlugin};

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
            FrameTimeDiagnosticsPlugin,
            chunk::ChunkPlugin,
            LookTransformPlugin,
            FpsCameraPlugin::default(),
            InputPlugin,
            DefaultInspectorConfigPlugin,
            EguiPlugin,
            TweeningPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            ui::inspector_ui.run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 2,
            first_cascade_far_bound: 4.0,
            maximum_distance: 80.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                smoothing_weight: 0.1,
                mouse_rotate_sensitivity: Vec2::splat(0.9),
                translate_sensitivity: 35.0,
                ..default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}
