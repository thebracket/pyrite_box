use crate::region::region_map::RegionMap;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub fn load_maps(path: &Path) -> Result<HashMap<usize, RegionMap>> {
    let index_path = path.join("index.ron");
    let index = MapIndex::load(&index_path)?;
    let mut maps = HashMap::<usize, RegionMap>::new();

    for mi in index.0.iter() {
        let map_path = path.join(&mi.filename);
        if !map_path.exists() {
            return Err(Error::msg(format!(
                "Map file does not exist: {:?}",
                map_path
            )));
        }
        let data = std::fs::read_to_string(map_path)?;
        let mut map: RegionMap = ron::from_str(&data)?;
        map.filename = path
            .join(&mi.filename)
            .to_str()
            .as_ref()
            .unwrap()
            .to_string()
            .replace("\\", "/");
        maps.insert(mi.index, map);
    }
    Ok(maps)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapIndexEntry {
    pub index: usize,
    pub filename: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapIndex(pub Vec<MapIndexEntry>);

impl MapIndex {
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&data)?)
    }
}
