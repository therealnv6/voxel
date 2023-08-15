use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active,
    pbr::wireframe::WireframePlugin,
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
use ui::inspector_ui;

pub mod chunk;
pub mod input;
pub mod ui;

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
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            inspector_ui.run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .run();
}

fn setup(mut commands: Commands, input: Res<Input<KeyCode>>) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                // no smoothing, we're just using this plugin because... well.. i'm lazy.
                enabled: input_toggle_active(true, KeyCode::Escape)(input),
                smoothing_weight: 0.0,
                mouse_rotate_sensitivity: Vec2::splat(1.5),
                translate_sensitivity: 16.0,
                ..default()
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}
