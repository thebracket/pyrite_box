use super::{material_loader::MaterialFile, ModuleHeader};
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
    for (_index, map) in module.maps.iter() {
        let filename = map.filename.clone();
        let map_ron = to_string_pretty(map, PrettyConfig::new())?;
        std::fs::write(Path::new(&filename), map_ron)?;
    }

    // Save materials
    for (index, (name, material, filename)) in module.materials.iter() {
        let mf = MaterialFile {
            index: *index,
            name: name.clone(),
            material: material.clone(),
        };
        let mat_ron = to_string_pretty(&mf, PrettyConfig::new())?;
        std::fs::write(Path::new(filename), mat_ron)?;
    }

    // Save scripts
    //let scripts_path = base_path.join("scripts");
    // For now, scripts must be edited by hand

    Ok(())
}
