use crate::module::MaterialDefinition;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

pub fn load_materials(path: &Path) -> Result<HashMap<usize, (String, MaterialDefinition, String)>> {
    let mut materials = HashMap::<usize, (String, MaterialDefinition, String)>::new();

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        if let Ok(material_path) = path {
            let data = std::fs::read_to_string(material_path.path())?;
            let material_file: MaterialFile = ron::from_str(&data)?;
            let filename = material_path.path().to_str().unwrap().to_string();
            materials.insert(
                material_file.index,
                (
                    material_file.name.clone(),
                    material_file.material,
                    filename.clone(),
                ),
            );
        }
    }

    Ok(materials)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MaterialFile {
    pub index: usize,
    pub name: String,
    pub material: MaterialDefinition,
}
