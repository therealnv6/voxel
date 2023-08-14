use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    // Get the single mutable window from the query
    let mut window = windows.single_mut();

    // Check if the left mouse button was just pressed
    if mouse.just_pressed(MouseButton::Left) {
        // Hide the cursor and set grab mode to locked
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    // Check if the escape key was just pressed
    if key.just_pressed(KeyCode::Escape) {
        // Show the cursor and set grab mode to none
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
