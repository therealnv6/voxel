use bevy::prelude::*;

use self::cursor::grab_mouse;

pub mod cursor;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grab_mouse);
    }
}
