use super::{
    map_loader::{MapIndex, MapIndexEntry},
    ModuleHeader, material_loader::MaterialFile,
};
use crate::module::Module;
use anyhow::{Error, Result};
use ron::ser::{to_string_pretty, PrettyConfig};
use std::{fs::create_dir, path::Path};

pub fn save_module(module: &Module) -> Result<()> {
    let base_path = Path::new(&module.base_path);
    if !base_path.exists() {
        create_dir(base_path)?;
        create_dir(base_path.join("maps"))?;
        create_dir(base_path.join("materials"))?;
        create_dir(base_path.join("scripts"))?;
    }
    if !base_path.is_dir() {
        return Err(Error::msg(
            "Module must be a directory. A file of the same name exists, so aborting.",
        ));
    }

    // Save header
    let header_path = base_path.join("header.ron");
    let header = ModuleHeader {
        name: module.name.clone(),
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
        map_index.0.push(MapIndexEntry {
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
    for (index, (name, material, filename)) in module.materials.iter() {
        let mf = MaterialFile{
            index: *index,
            name: name.clone(),
            material: material.clone(),
        };
        let mat_ron = to_string_pretty(&mf, PrettyConfig::new())?;
        std::fs::write(Path::new(filename), mat_ron)?;
    }

    // Save scripts
    let scripts_path = base_path.join("scripts");
    // TODO

    Ok(())
}
