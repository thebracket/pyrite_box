use anyhow::Result;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterHeader {
    pub name: String,
}

impl CharacterHeader {
    pub fn new() -> Self {
        Self {
            name: "New Character".to_string(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Path::new("characters").join(&format!("{}.chr", &self.name));
        println!("{:?}", path);
        let header_ron = to_string_pretty(&self, PrettyConfig::new())?;
        std::fs::write(path, header_ron)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<CharacterHeader> {
        let data = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&data)?)
    }

    pub fn scan_available() -> Vec<Self> {
        let paths = fs::read_dir("./characters").unwrap();
        paths
            .flatten()
            .flat_map(|path| {
                if !path.path().is_dir() {
                    let chr = CharacterHeader::load(&path.path());
                    if chr.is_err() {
                        None
                    } else {
                        Some(chr.unwrap())
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
