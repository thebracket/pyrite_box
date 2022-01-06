use crate::module::game_events::GameEvent;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn load_scripts(path: &Path) -> Result<Vec<GameEvent>> {
    let paths = fs::read_dir(path).unwrap();

    Ok(paths
        .flatten()
        .map(|script_path| {
            let data = std::fs::read_to_string(script_path.path())?;
            let script = ron::from_str::<Vec<GameEvent>>(&data)?;
            Ok(script)
        })
        .collect::<Result<Vec<Vec<_>>>>()?
        .into_iter()
        .flatten()
        .collect())
}
