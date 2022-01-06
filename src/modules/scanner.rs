use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleHeader {
    pub name: String,
    pub description: String,
    pub author: String,
    pub filename: Option<Box<Path>>,
    pub module_start_event: String,
    pub starting_map_idx: usize,
    pub ui_images: Vec<(String, String)>,
    pub sprites: Vec<(String, String)>,
}

impl ModuleHeader {
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&data)?)
    }
}

pub fn list_available_modules() -> Vec<ModuleHeader> {
    let paths = fs::read_dir("./modules").unwrap();
    paths
        .flatten()
        .flat_map(|path| {
            {
                if path.path().is_dir() {
                    let header = path.path().join("header.ron");
                    if header.exists() {
                        let header_data = ModuleHeader::load(&header);
                        if let Ok(mut header_unwrap) = header_data {
                            header_unwrap.filename = Some(path.path().into_boxed_path());
                            Some(header_unwrap)
                        } else {
                            println!("While loading {:?}:", path.path());
                            println!("{:#?}", header_data);
                            //FIXME: is this an error?
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        })
        .collect()
}
