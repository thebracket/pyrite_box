use std::path::Path;
use std::fs;
use anyhow::Result;
use crate::module::game_events::GameEvent;

pub fn load_scripts(path: &Path) -> Result<Vec<GameEvent>> {
    let mut result = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        if let Ok(script_path) = path {
            let data = std::fs::read_to_string(script_path.path())?;
            let script = ron::from_str::<Vec<GameEvent>>(&data)?;
            result.extend_from_slice(&script);
        }
    }
    Ok(result)
}