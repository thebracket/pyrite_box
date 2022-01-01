use crate::module::MaterialDefinition;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub fn load_materials(path: &Path) -> Result<HashMap<usize, (String, MaterialDefinition, String)>> {
    let index_path = path.join("index.ron");
    let index = MaterialIndex::load(&index_path)?;
    let mut materials = HashMap::<usize, (String, MaterialDefinition, String)>::new();

    for mi in index.0.iter() {
        let mat_path = path.join(&mi.filename);
        if !mat_path.exists() {
            return Err(Error::msg(format!(
                "Material file does not exist: {:?}",
                mat_path
            )));
        }
        let data = std::fs::read_to_string(mat_path)?;
        let material = ron::from_str(&data)?;
        materials.insert(mi.index, (mi.name.clone(), material, mi.filename.clone()));
    }

    Ok(materials)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MaterialIndexEntry {
    pub index: usize,
    pub filename: String,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MaterialIndex(pub Vec<MaterialIndexEntry>);

impl MaterialIndex {
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&data)?)
    }
}
