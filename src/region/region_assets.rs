use crate::module::{MaterialDefinition, Module};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct RegionAssets {
    pub materials: HashMap<usize, Handle<StandardMaterial>>,
    pub meshes: Vec<(u32, Handle<Mesh>)>,
}

impl RegionAssets {
    pub fn new(
        materials: &mut Assets<StandardMaterial>,
        meshes: &mut Assets<Mesh>,
        asset_server: &AssetServer,
        module: &Module,
        map_idx: usize,
    ) -> Self {
        let mut mats = HashMap::new();
        for (idx, (_name, mat)) in module.materials.iter() {
            match mat {
                MaterialDefinition::Color { r, g, b } => {
                    let handle = materials.add(
                        Color::rgb(*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0).into(),
                    );
                    mats.insert(*idx, handle.clone());
                }
                MaterialDefinition::Pbr {
                    display_color: _,
                    albedo,
                    roughness,
                    metallic,
                    normal_map,
                    occlusion,
                    metallic_roughness_texture,
                    emissive,
                } => {
                    let material = StandardMaterial {
                        base_color: Color::rgb(1.0, 1.0, 1.0),
                        base_color_texture: if albedo.is_empty() {
                            None
                        } else {
                            Some(asset_server.load(albedo.as_str()))
                        },
                        roughness: *roughness,
                        metallic: *metallic,
                        normal_map: if normal_map.is_empty() {
                            None
                        } else {
                            Some(asset_server.load(normal_map.as_str()))
                        },
                        occlusion_texture: if occlusion.is_empty() {
                            None
                        } else {
                            Some(asset_server.load(occlusion.as_str()))
                        },
                        metallic_roughness_texture: if metallic_roughness_texture.is_empty() {
                            None
                        } else {
                            Some(asset_server.load(metallic_roughness_texture.as_str()))
                        },
                        emissive_texture: if emissive.is_empty() {
                            None
                        } else {
                            Some(asset_server.load(emissive.as_str()))
                        },
                        ..Default::default()
                    };
                    let handle = materials.add(material);
                    mats.insert(*idx, handle.clone());
                }
            }
        }

        let meshes = module.maps[&map_idx].create_geometry(meshes);

        Self {
            materials: mats,
            meshes,
        }
    }

    pub fn rebuild_geometry(&mut self, meshes: &mut Assets<Mesh>, module: &Module, map_idx: usize) {
        for mh in self.meshes.iter() {
            meshes.remove(mh.1.clone());
        }
        self.meshes = module.maps[&map_idx].create_geometry(meshes);
    }
}
