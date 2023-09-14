use bevy::{
    core_pipeline::fxaa::Fxaa,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
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
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            chunk::ChunkPlugin,
            world::WorldPlugin,
            InputPlugin,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_fps_text_sys)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: (70.0 / 360.0) * (std::f32::consts::PI * 2.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        Fxaa::default(),
        PlayerController::default(),
    ));

    commands.spawn((
        TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                }],
                ..default()
            },
            ..default()
        },
        TopRightText,
    ));
}

#[derive(Component)]
struct TopRightText;

fn update_fps_text_sys(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<TopRightText>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = 0.0f64;
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        let text = &mut text.sections[0].value;
        text.clear();
        *text = format!("{:.1} fps, {:.3} ms/frame", fps, frame_time);
    }
}
