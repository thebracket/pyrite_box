use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleHeader {
    pub name: String,
    pub description: String,
    pub author: String,
    pub filename: Option<Box<Path>>,
    pub module_start_event: String,
    pub starting_map_idx: usize,
}

impl ModuleHeader {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&data)?)
    }
}

pub fn list_available_modules() -> Vec<ModuleHeader> {
    let mut result = Vec::new();

    let paths = fs::read_dir("./modules").unwrap();
    for path in paths {
        if let Ok(path) = path {
            if path.path().is_dir() {
                let header = path.path().join("header.ron");
                if header.exists() {
                    let header_data = ModuleHeader::load(&header);
                    if let Ok(mut header_unwrap) = header_data {
                        header_unwrap.filename = Some(path.path().into_boxed_path());
                        result.push(header_unwrap);
                    } else {
                        println!("While loading {:?}:", path.path());
                        println!("{:#?}", header_data);
                    }
                }
            }
        }
    }

    result
}
