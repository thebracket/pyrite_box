use crate::{
    region::{region_assets::RegionAssets, region_map::RegionMap},
};
use bevy::prelude::*;

pub struct MapWander {}

pub fn map_wander() {}

pub fn resume_map_wander(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = RegionMap::default();
    let (start_x, start_y, start_z) = map.starting_location;
    let assets = RegionAssets::new(&mut materials, &mut meshes, &map);
    //commands.insert_resource(assets);
    for m in assets.meshes.iter() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: m.clone(),
                material: assets.green.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(MapWander {});
    }

    // light
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(start_x, start_y, start_z),
            ..Default::default()
        })
        .insert(MapWander {});

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(start_x, start_y, start_z)
                .looking_at(Vec3::new(start_x, start_y + 50.0, start_z), Vec3::Z),
            ..Default::default()
        })
        .insert(MapWander {});
}

pub fn exit_map_wander(
    mut commands: Commands,
    cleanup: Query<(Entity, &MapWander)>) 
{
        cleanup
            .iter()
            .for_each(|(e, _)| commands.entity(e).despawn());

}
