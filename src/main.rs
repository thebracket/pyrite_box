use bevy::prelude::*;
use region::{region_assets::RegionAssets, region_map::RegionMap};
mod region;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = RegionMap::default();
    let (start_x, start_y, start_z) = map.starting_location;
    let assets = RegionAssets::new(&mut materials, &mut meshes, &map);
    //commands.insert_resource(assets);
    for m in assets.meshes.iter() {
        commands.spawn_bundle(PbrBundle {
            mesh: m.clone(),
            material: assets.green.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
    }

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(start_x, start_y, start_z),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(start_x, start_y, start_z + 20.0)
            .looking_at(Vec3::new(start_x, start_y + 50.0, start_z), Vec3::Z),
        ..Default::default()
    });
}
