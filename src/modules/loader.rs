use std::path::Path;
use anyhow::{Result, Error};
use crate::{modules::{material_loader::load_materials, map_loader::load_maps, scripts_loader::load_scripts}, module::Module};
use super::ModuleHeader;

pub fn load_module(path: &Path) -> Result<Module> {
    println!("{:?}", path);
    if !path.exists() {
        return Err(Error::msg("Module path not found"));
    }
    if !path.is_dir() {
        return Err(Error::msg("Modules must be a directory"));
    }

    // Load the header
    let header_path = path.join("header.ron");
    let header = ModuleHeader::load(&header_path)?;

    // Maps directory
    check_directory(path, "maps")?;
    let map_path = path.join("maps");
    let maps = load_maps(&map_path)?;

    // Scripts directory
    check_directory(path, "scripts")?;
    let scripts_path = path.join("scripts");
    let scripts = load_scripts(&scripts_path)?;

    // Materials directory
    check_directory(path, "materials")?;
    let mat_path = path.join("materials");
    let materials = load_materials(&mat_path)?;

    let next_material_index = materials.keys().max().unwrap()+1;
    let next_map_index = maps.keys().max().unwrap()+1;

    let module = Module {
        name : header.name,
        description: header.description,
        module_start_event: header.module_start_event,
        starting_map_idx: header.starting_map_idx,
        materials,
        maps,
        events: scripts,
        next_material_index,
        next_map_index
    };

    Ok(module)
}

fn check_directory(path: &Path, directory: &str) -> Result<()> {
    let dir_path = path.join(directory);
    if !dir_path.exists() {
        return Err(Error::msg(format!("{} path not found", directory)));
    }
    if !dir_path.is_dir() {
        return Err(Error::msg(format!("{} must be a directory", directory)));
    }
    Ok(())
}