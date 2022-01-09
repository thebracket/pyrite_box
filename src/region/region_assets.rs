use crate::{
    module::{MaterialDefinition, Module},
    region::region_map::geometry::GEOMETRY_SIZE,
};
use bevy::{
    prelude::*,
    render::{mesh::VertexAttributeValues, render_resource::PrimitiveTopology},
};
use bevy_egui::{egui::TextureId, EguiContext};
use std::collections::HashMap;

pub struct RegionAssets {
    pub materials: HashMap<usize, Handle<StandardMaterial>>,
    pub meshes: Vec<(u32, Handle<Mesh>)>,
    pub ui_images: HashMap<String, TextureId>,
    pub sprites: HashMap<String, Handle<StandardMaterial>>,
    pub sprite_mesh: Handle<Mesh>,
    pub battle_tile_atlas: Handle<TextureAtlas>,
}

impl RegionAssets {
    pub fn new(
        materials: &mut Assets<StandardMaterial>,
        meshes: &mut Assets<Mesh>,
        asset_server: &AssetServer,
        module: &Module,
        map_idx: usize,
        egui: &mut EguiContext,
        handles: &mut Vec<HandleUntyped>, // Used for tracking image loading
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut mats = HashMap::new();
        for (idx, (_name, mat, _)) in module.materials.iter() {
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
                            let handle = asset_server.load(albedo.as_str());
                            handles.push(handle.clone_untyped());
                            Some(handle)
                        },
                        perceptual_roughness: *roughness,
                        metallic: *metallic,
                        normal_map_texture: if normal_map.is_empty() {
                            None
                        } else {
                            let handle = asset_server.load(normal_map.as_str());
                            handles.push(handle.clone_untyped());
                            Some(handle)
                        },
                        occlusion_texture: if occlusion.is_empty() {
                            None
                        } else {
                            let handle = asset_server.load(normal_map.as_str());
                            handles.push(handle.clone_untyped());
                            Some(handle)
                        },
                        metallic_roughness_texture: if metallic_roughness_texture.is_empty() {
                            None
                        } else {
                            let handle = asset_server.load(normal_map.as_str());
                            handles.push(handle.clone_untyped());
                            Some(handle)
                        },
                        emissive_texture: if emissive.is_empty() {
                            None
                        } else {
                            let handle = asset_server.load(normal_map.as_str());
                            handles.push(handle.clone_untyped());
                            Some(handle)
                        },
                        ..Default::default()
                    };
                    let handle = materials.add(material);
                    mats.insert(*idx, handle.clone());
                }
            }
        }

        let map_meshes = module.maps[&map_idx].create_geometry(meshes);

        // Load the UI images
        let mut ui_images = HashMap::new();
        for (i, (key, file)) in module.ui_images.iter().enumerate() {
            let image_id = asset_server.load(file.as_str());
            egui.set_egui_texture(i as u64, image_id);
            ui_images.insert(key.clone(), bevy_egui::egui::TextureId::User(i as u64));
        }

        // Load sprites
        let mut sprites = HashMap::new();
        for (key, file) in module.sprites.iter() {
            let image_handle = asset_server.load(file.as_str());
            handles.push(image_handle.clone_untyped());
            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(image_handle),
                alpha_mode: AlphaMode::Mask(0.9),
                double_sided: true,
                ..Default::default()
            });
            sprites.insert(key.clone(), material_handle.clone());
        }

        // Reusable Sprite Mesh
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        const HALF: f32 = GEOMETRY_SIZE / 2.0;
        let x1 = 0.0;
        let y0 = -HALF;
        let y1 = HALF;
        let z0 = -HALF;
        let z1 = HALF;
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vec![
                [x1, y1, z1],
                [x1, y1, z0],
                [x1, y0, z0],
                [x1, y0, z0],
                [x1, y0, z1],
                [x1, y1, z1],
            ]),
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(vec![
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
            ]),
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(vec![
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 1.0],
                [1.0, 1.0],
                [1.0, 0.0],
                [0.0, 0.0],
            ]),
        );
        let sprite_mesh = meshes.add(mesh);

        // Battle tile atlas - TODO: make this data-driven
        let tile_handle = asset_server.load("battle-sprites.png");
        handles.push(tile_handle.clone_untyped());
        let texture_atlas =
            TextureAtlas::from_grid(tile_handle.clone(), Vec2::new(32.0, 32.0), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Self {
            materials: mats,
            meshes: map_meshes,
            ui_images,
            sprites,
            sprite_mesh,
            battle_tile_atlas: texture_atlas_handle,
        }
    }

    pub fn rebuild_geometry(&mut self, meshes: &mut Assets<Mesh>, module: &Module, map_idx: usize) {
        for mh in self.meshes.iter() {
            meshes.remove(mh.1.clone());
        }
        self.meshes = module.maps[&map_idx].create_geometry(meshes);
    }
}
