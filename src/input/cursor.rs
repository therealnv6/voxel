use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use smooth_bevy_cameras::controllers::fps::FpsCameraController;

pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mut camera: Query<&mut FpsCameraController>,
    key: Res<Input<KeyCode>>,
) {
    // Get the single mutable window from the query
    let mut window = windows.single_mut();
    let mut camera = camera.single_mut();

    if key.just_pressed(KeyCode::AltLeft) {
        window.cursor.visible = camera.enabled;
        camera.enabled = !camera.enabled;

        window.cursor.grab_mode = match window.cursor.visible {
            true => CursorGrabMode::None,
            false => CursorGrabMode::Locked,
        };
    }
}
