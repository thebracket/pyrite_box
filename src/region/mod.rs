use serde::{Deserialize, Serialize};
pub mod region_assets;
pub mod region_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
