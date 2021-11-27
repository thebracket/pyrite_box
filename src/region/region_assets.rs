use bevy::prelude::*;

use super::region_map::RegionMap;

pub struct RegionAssets {
    pub materials: Vec<Handle<StandardMaterial>>,
    pub meshes: Vec<(u32, Handle<Mesh>)>,
}

impl RegionAssets {
    pub fn new(
        materials: &mut Assets<StandardMaterial>,
        meshes: &mut Assets<Mesh>,
        map: &RegionMap,
    ) -> Self {
        let green = materials.add(Color::rgb(0.0, 1.0, 0.0).into());
        let grey = materials.add(Color::rgb(0.7, 0.7, 0.7).into());

        let meshes = map.create_geometry(meshes);

        Self {
            materials: vec![green.clone(), grey.clone()],
            meshes,
        }
    }
}
