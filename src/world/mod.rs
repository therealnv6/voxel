use bevy::prelude::*;

pub mod sky;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sky::setup_sky_lighting);
        app.add_systems(Update, sky::update_light_position);
    }
}
