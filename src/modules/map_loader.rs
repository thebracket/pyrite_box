use crate::region::region_map::RegionMap;
use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};

pub fn load_maps(path: &Path) -> Result<HashMap<usize, RegionMap>> {
    let mut maps = HashMap::<usize, RegionMap>::new();

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        if let Ok(map_path) = path {
            //println!("{:?}", map_path.path());
            let data = std::fs::read_to_string(map_path.path())?;
            let mut map_file: RegionMap = ron::from_str(&data)?;
            let filename = map_path.path().to_str().unwrap().to_string();
            map_file.filename = filename;
            maps.insert(map_file.index, map_file);
        }
    }
    Ok(maps)
}
