use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
mod materials;
pub use materials::*;

/// Represents an adventure module, bundling all assets together.
#[derive(Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub description: String,
    pub filename: String,
    pub materials: Vec<(String, MaterialDefinition)>,
}

impl Module {
    pub fn default() -> Self {
        let mut materials = Vec::new();
        materials.push((
            "Green".to_string(),
            MaterialDefinition::Color { r: 0, g: 255, b: 0 },
        ));
        materials.push((
            "Gray".to_string(),
            MaterialDefinition::Color {
                r: 128,
                g: 128,
                b: 128,
            },
        ));

        Self {
            name: "New Module".to_string(),
            description: String::new(),
            filename: "TestModule.pyr".to_string(),
            materials,
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
