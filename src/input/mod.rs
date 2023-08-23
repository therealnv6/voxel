use bevy::prelude::*;

pub mod camera;
pub mod cursor;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                cursor::grab_mouse,
                camera::handle_mouse,
                camera::handle_move,
            ),
        );
    }
}
