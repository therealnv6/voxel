use bevy::prelude::*;

use crate::input::camera::PlayerController;

#[derive(Resource, Deref)]
pub struct SkyLightEntity(Entity);

pub fn setup_sky_lighting(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::IDENTITY.looking_to(Vec3::new(-1.0, -0.5, -1.0), Vec3::Y),
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.7,
    });
}

pub fn update_light_position(
    mut queries: ParamSet<(
        Query<&Transform, With<PlayerController>>,
        Query<&mut Transform, With<DirectionalLight>>,
    )>,
) {
    let translation = queries
        .p0()
        .get_single()
        .map_or_else(|_| Default::default(), |player| player.translation);

    let mut binding = queries.p1();
    let mut transform = binding.single_mut();

    transform.translation = translation;
}
