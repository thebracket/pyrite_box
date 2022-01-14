use super::scaling::Scaling;
use crate::{region::region_map::RegionMap, utils::Direction};
use bevy_egui::egui::Pos2;

pub struct MapWallInteraction {
    pub tile_x: u32,
    pub tile_y: u32,
    pub selected_wall: Option<Direction>,
}

impl MapWallInteraction {
    pub fn new(scale: &Scaling, pointer_pos: Pos2, map: &RegionMap) -> Self {
        let canvas_pos = scale.from_screen * pointer_pos;
        let tile_x = f32::min(
            (canvas_pos.x * map.size.0 as f32).floor(),
            map.size.0 as f32 - 1.0,
        );
        let tile_y = f32::min(
            (canvas_pos.y * map.size.0 as f32).floor(),
            map.size.1 as f32 - 1.0,
        );

        let wall = if canvas_pos.x < (tile_x * scale.box_x) + (10.0 * scale.x10) {
            Some(Direction::West)
        } else if canvas_pos.x > ((tile_x + 1.0) * scale.box_x) - (10.0 * scale.x10) {
            Some(Direction::East)
        } else if canvas_pos.y < (tile_y * scale.box_y) + (10.0 * scale.y10) {
            Some(Direction::North)
        } else if canvas_pos.y > ((tile_y + 1.0) * scale.box_y) - (10.0 * scale.y10) {
            Some(Direction::South)
        } else {
            None
        };

        Self {
            tile_x: tile_x as u32,
            tile_y: tile_y as u32,
            selected_wall: wall,
        }
    }
}
