use std::{path::Path, collections::HashMap};
use anyhow::{Result, Error};
use serde::{Deserialize, Serialize};
use crate::region::region_map::RegionMap;

pub fn load_maps(path: &Path) -> Result<HashMap::<usize, RegionMap>> {
    let index_path = path.join("index.ron");
    let index = MapIndex::load(&index_path)?;
    let mut maps = HashMap::<usize, RegionMap>::new();

    for mi in index.0.iter() {
        let map_path = path.join(&mi.filename);
        if !map_path.exists() {
            return Err(Error::msg(format!("Map file does not exist: {:?}", map_path)));
        }
        let data = std::fs::read_to_string(map_path)?;
        let map = ron::from_str(&data)?;
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
pub struct MapIndex(Vec<MapIndexEntry>);

impl MapIndex {
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&data)?)
    }
}