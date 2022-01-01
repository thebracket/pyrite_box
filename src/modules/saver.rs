use std::{path::Path, fs::create_dir};
use anyhow::{Result, Error};
use crate::module::Module;
use super::{ModuleHeader, map_loader::{MapIndex, MapIndexEntry}, material_loader::{MaterialIndex, MaterialIndexEntry}};
use ron::ser::{to_string_pretty, PrettyConfig};

pub fn save_module(module: &Module) -> Result<()> {
    let base_path = Path::new(&module.base_path);
    if !base_path.exists() {
        create_dir(base_path)?;
        create_dir(base_path.join("maps"))?;
        create_dir(base_path.join("materials"))?;
        create_dir(base_path.join("scripts"))?;
    }
    if !base_path.is_dir() {
        return Err(Error::msg("Module must be a directory. A file of the same name exists, so aborting."));
    }

    // Save header
    let header_path = base_path.join("header.ron");
    let header = ModuleHeader {
        name : module.name.clone(),
        description: module.description.clone(),
        author: module.author.clone(),
        filename: None,
        module_start_event: module.module_start_event.clone(),
        starting_map_idx: module.starting_map_idx,
    };
    let header_ron = to_string_pretty(&header, PrettyConfig::new())?;
    std::fs::write(header_path, header_ron)?;

    // Save maps
    let maps_index_path = base_path.join("maps").join("index.ron");
    let mut map_index = MapIndex(Vec::new());
    for map in module.maps.iter() {
        map_index.0.push(MapIndexEntry{
            index: *map.0,
            filename: map.1.filename.clone(),
        });
        let map_file = Path::new(&map.1.filename);
        let map_ron = to_string_pretty(map.1, PrettyConfig::new())?;
        std::fs::write(map_file, map_ron)?;
    }
    let map_index_ron = to_string_pretty(&map_index, PrettyConfig::new())?;
    std::fs::write(maps_index_path, map_index_ron)?;

    // Save materials
    let mats_index_path = base_path.join("materials").join("index.ron");
    let mut mat_index = MaterialIndex(Vec::new());
    for mat in module.materials.iter() {
        mat_index.0.push(MaterialIndexEntry {
            index: *mat.0,
            filename: mat.1.2.clone(),
            name: mat.1.0.clone(),
        });
    }
    let mat_index_ron = to_string_pretty(&mat_index, PrettyConfig::new())?;
    std::fs::write(mats_index_path, mat_index_ron)?;

    // Save scripts
    let scripts_path = base_path.join("scripts");

    Ok(())
}