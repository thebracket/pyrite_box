use bevy::prelude::*;

use super::region_map::RegionMap;

pub struct RegionAssets {
    pub green: Handle<StandardMaterial>,
    pub meshes: Vec<Handle<Mesh>>,
}

impl RegionAssets {
    pub fn new(
        materials: &mut Assets<StandardMaterial>,
        meshes: &mut Assets<Mesh>,
        map: &RegionMap,
    ) -> Self {
        let green = materials.add(Color::rgb(0.0, 1.0, 0.0).into());
        let meshes = map.create_geometry(meshes);

        Self {
            green: green.clone(),
            meshes,
        }
    }
}
