use crate::region::region_map::RegionMap;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::MaterialDefinition;

/// Represents an adventure module, bundling all assets together.
#[derive(Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub description: String,
    pub filename: String,
    pub materials: HashMap<usize, (String, MaterialDefinition)>,
    pub next_material_index: usize,
    pub maps: HashMap<usize, RegionMap>,
    pub next_map_index: usize,
}

impl Module {
    pub fn default() -> Self {
        let mut materials = HashMap::new();
        materials.insert(
            0,
            (
                "Green".to_string(),
                MaterialDefinition::Color { r: 0, g: 255, b: 0 },
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
            ),
        );

        Self {
            name: "New Module".to_string(),
            description: String::new(),
            filename: "TestModule.pyr".to_string(),
            materials,
            next_material_index: 2,
            maps: HashMap::new(),
            next_map_index: 0,
        }
    }

    pub fn load(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        ron::from_str(&data).expect("Deserialize fail")
    }

    pub fn save(&self) {
        // We'll move to a concise format once the contents is stable
        let data = to_string_pretty(&self, PrettyConfig::default()).expect("Serialization fail");
        std::fs::write(&self.filename, data).expect("Save fail");
    }
}
