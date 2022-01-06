use super::{game_events::EventList, MaterialDefinition};
use crate::region::region_map::RegionMap;
use std::collections::HashMap;

/// Represents an adventure module, bundling all assets together.
#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub description: String,
    pub author: String,
    pub materials: HashMap<usize, (String, MaterialDefinition, String)>,
    pub next_material_index: usize,
    pub maps: HashMap<usize, RegionMap>,
    pub next_map_index: usize,
    pub events: EventList,
    pub module_start_event: String,
    pub starting_map_idx: usize,
    pub base_path: String,
    pub ui_images: Vec<(String, String)>,
}

impl Module {
    pub fn default() -> Self {
        let mut materials = HashMap::new();
        materials.insert(
            0,
            (
                "Green".to_string(),
                MaterialDefinition::Color { r: 0, g: 255, b: 0 },
                "green.ron".to_string(),
            ),
        );
        materials.insert(
            1,
            (
                "Gray".to_string(),
                MaterialDefinition::Color {
                    r: 128,
                    g: 128,
                    b: 128,
                },
                "gray.ron".to_string(),
            ),
        );

        Self {
            name: "New Module".to_string(),
            description: String::new(),
            author: String::new(),
            materials,
            next_material_index: 2,
            maps: HashMap::new(),
            next_map_index: 0,
            events: EventList {
                filename: "scripts.ron".to_string(),
                events: Vec::new(),
            },
            module_start_event: String::new(),
            starting_map_idx: 0,
            base_path: "./modules/NewModule".to_string(),
            ui_images: Vec::new(),
        }
    }

    pub fn save(&self) {
        crate::modules::save_module(self).unwrap();
    }
}
