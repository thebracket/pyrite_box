use super::BattleComponent;
use crate::{
    game_states::WanderingPlayer,
    module::Direction,
    region::{
        region_assets::RegionAssets,
        region_map::{RegionBoundaryType, RegionMap, RegionTileType},
    },
};
use bevy::prelude::*;

pub const BATTLE_WIDTH: usize = 24;
pub const BATTLE_HEIGHT: usize = 24;
pub const TILE_WIDTH: usize = BATTLE_WIDTH / 3;
pub const TILE_HEIGHT: usize = BATTLE_HEIGHT / 3;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BattleTile {
    Open,
    Wall,
}

pub struct BattleMap {
    pub tiles: Vec<BattleTile>,
    pub region_coords: (i32, i32),
}

impl BattleMap {
    /// Creates a new, empty battle map structure.
    /// This won't be a lot of use without chaining generator functions.
    pub fn new() -> Self {
        Self {
            tiles: vec![BattleTile::Open; BATTLE_WIDTH * BATTLE_HEIGHT],
            region_coords: (0, 0),
        }
    }

    /// Obtain a tile index on the battle map, from x/y TILE coordinates
    pub fn battle_tile_idx(&self, x: usize, y: usize) -> usize {
        (y * BATTLE_WIDTH) + x
    }

    pub fn setup_region_coordinates(mut self, player: &WanderingPlayer) -> Self {
        self.region_coords.0 = player.x - 1;
        self.region_coords.1 = player.y - 1;
        self
    }

    pub fn build_from_region(mut self, map: &RegionMap) -> Self {
        for y in self.region_coords.1..self.region_coords.1 + 3 {
            for x in self.region_coords.0..self.region_coords.0 + 3 {
                if x < map.size.0 as i32 && y < map.size.1 as i32 && x >= 0 && y >= 0 {
                    let region_idx = ((y * map.size.1 as i32) + x) as usize;
                    match map.tiles[region_idx].tile_type {
                        RegionTileType::Solid => self.make_region_tile_solid(x, y),
                        _ => {
                            // Solid
                            self.make_region_tile_solid(x, y);
                            // Carve out center
                            self.make_region_tile_open(x, y);
                            // Clear exits
                            self.process_exits(map, x, y, region_idx);
                        }
                    }
                } else {
                    // Out of bounds - make it unavailable
                    self.make_region_tile_solid(x, y);
                }
            }
        }
        self
    }

    fn make_region_tile_solid(&mut self, x: i32, y: i32) {
        for by in 0..TILE_HEIGHT {
            for bx in 0..TILE_WIDTH {
                let idx = self.battle_tile_idx(
                    bx + ((x - self.region_coords.0) as usize * TILE_WIDTH),
                    by + ((y - self.region_coords.1) as usize * TILE_HEIGHT),
                );
                self.tiles[idx] = BattleTile::Wall;
            }
        }
    }

    fn make_region_tile_open(&mut self, x: i32, y: i32) {
        for by in 1..TILE_HEIGHT - 1 {
            for bx in 1..TILE_WIDTH - 1 {
                let idx = self.battle_tile_idx(
                    bx + ((x - self.region_coords.0) as usize * TILE_WIDTH),
                    by + ((y - self.region_coords.1) as usize * TILE_HEIGHT),
                );
                self.tiles[idx] = BattleTile::Open;
            }
        }
    }

    fn process_exits(&mut self, map: &RegionMap, x: i32, y: i32, idx: usize) {
        let top_left = self.battle_tile_idx(
            (x - self.region_coords.0) as usize * TILE_WIDTH,
            (y - self.region_coords.1) as usize * TILE_HEIGHT,
        );

        // North
        match map.tiles[idx].boundaries[Direction::North.to_exit_index()].0 {
            RegionBoundaryType::None => {
                for x in 1..TILE_WIDTH - 1 {
                    self.tiles[top_left + x] = BattleTile::Open;
                }
            }
            RegionBoundaryType::Opening => {
                for x in 1..TILE_WIDTH - 1 {
                    let distance = (TILE_WIDTH as i32 / 2 - x as i32).abs();
                    if distance < 2 {
                        self.tiles[top_left + x] = BattleTile::Open;
                    }
                }
            }
            _ => {} // Do nothing
        }

        // South
        let bottom_left = top_left + (BATTLE_WIDTH * (TILE_HEIGHT - 1));
        match map.tiles[idx].boundaries[Direction::South.to_exit_index()].0 {
            RegionBoundaryType::None => {
                for x in 1..TILE_WIDTH - 1 {
                    self.tiles[bottom_left + x] = BattleTile::Open;
                }
            }
            RegionBoundaryType::Opening => {
                for x in 1..TILE_WIDTH - 1 {
                    let distance = (TILE_WIDTH as i32 / 2 - x as i32).abs();
                    if distance < 2 {
                        self.tiles[bottom_left + x] = BattleTile::Open;
                    }
                }
            }
            _ => {} // Do nothing
        }

        // West
        match map.tiles[idx].boundaries[Direction::West.to_exit_index()].0 {
            RegionBoundaryType::None => {
                for y in 1..TILE_HEIGHT - 1 {
                    self.tiles[top_left + (y * BATTLE_WIDTH)] = BattleTile::Open;
                }
            }
            RegionBoundaryType::Opening => {
                for y in 1..TILE_HEIGHT - 1 {
                    let distance = (TILE_HEIGHT as i32 / 2 - y as i32).abs();
                    if distance < 2 {
                        self.tiles[top_left + (y * BATTLE_WIDTH)] = BattleTile::Open;
                    }
                }
            }
            _ => {} // Do nothing
        }

        // East
        match map.tiles[idx].boundaries[Direction::East.to_exit_index()].0 {
            RegionBoundaryType::None => {
                for y in 1..TILE_HEIGHT - 1 {
                    self.tiles[top_left + (y * BATTLE_WIDTH) + (TILE_WIDTH - 1)] = BattleTile::Open;
                }
            }
            RegionBoundaryType::Opening => {
                for y in 1..TILE_HEIGHT - 1 {
                    let distance = (TILE_HEIGHT as i32 / 2 - y as i32).abs();
                    if distance < 2 {
                        self.tiles[top_left + (y * BATTLE_WIDTH) + (TILE_WIDTH - 1)] =
                            BattleTile::Open;
                    }
                }
            }
            _ => {} // Do nothing
        }
    }

    fn tile_to_screen(&self, x: usize, y: usize) -> Vec2 {
        let mut result = Vec2::new(x as f32 * 32.0, y as f32 * 32.0)
            - Vec2::new(
                (BATTLE_WIDTH as f32 / 2.0) * 32.0,
                (BATTLE_HEIGHT as f32 / 2.0) * 32.0,
            );
        result.y = 0.0 - result.y;
        result
    }

    pub fn spawn_map_tiles(&self, commands: &mut Commands, assets: &RegionAssets) {
        for y in 0..BATTLE_HEIGHT {
            for x in 0..BATTLE_WIDTH {
                let coords = self.tile_to_screen(x, y);
                let index = match self.tiles[self.battle_tile_idx(x, y)] {
                    BattleTile::Open => 0,
                    BattleTile::Wall => 1,
                };
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: assets.battle_tile_atlas.clone(),
                        transform: Transform::from_xyz(coords.x, coords.y, 0.0),
                        sprite: TextureAtlasSprite {
                            index,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(BattleComponent {});
            }
        }
    }
}
