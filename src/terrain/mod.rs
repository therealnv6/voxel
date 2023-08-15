use bevy::prelude::*;
use fuss::Simplex;

pub struct TerrainPlugin;

#[derive(Resource)]
pub struct SimplexResource(Simplex);

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimplexResource(Simplex::new()));
    }
}

pub fn random_simplex(simplex: &Simplex) -> Vec<Vec<Vec<f32>>> {
    let mut map = Vec::<Vec<Vec<f32>>>::new();
    for x in 0..10 {
        map.push(Vec::<Vec<f32>>::new());
        for y in 0..10 {
            map[x as usize].push(Vec::<f32>::new());
            for z in 0..10 {
                map[x as usize][y as usize].push(simplex.noise_3d(x as f32, y as f32, z as f32));
            }
        }
    }

    return map;
}
