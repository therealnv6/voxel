use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use super::camera::PlayerController;

pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mut camera: Query<&mut PlayerController>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();
    let mut controller = camera.single_mut();

    if key.just_pressed(KeyCode::AltLeft) {
        window.cursor.visible = controller.locked;
        controller.locked = !controller.locked;

        window.cursor.grab_mode = match window.cursor.visible {
            true => CursorGrabMode::None,
            false => CursorGrabMode::Locked,
        };
    }
}
