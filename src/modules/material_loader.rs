use crate::module::MaterialDefinition;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

pub fn load_materials(path: &Path) -> Result<HashMap<usize, (String, MaterialDefinition, String)>> {
    let paths = fs::read_dir(path).unwrap();
    paths
        .flatten()
        .map(|material_path| {
            let data = std::fs::read_to_string(material_path.path())?;
            let material_file: MaterialFile = ron::from_str(&data)?;
            let filename = material_path.path().to_str().unwrap().to_string();
            Ok((
                material_file.index,
                (material_file.name.clone(), material_file.material, filename),
            ))
        })
        .collect()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MaterialFile {
    pub index: usize,
    pub name: String,
    pub material: MaterialDefinition,
}
