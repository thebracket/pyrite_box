use crate::{
    game_states::WanderingPlayer,
    module::Direction,
    region::region_map::{RegionBoundaryType, RegionMap, RegionTileType},
};

pub const BATTLE_WIDTH: usize = 32;
pub const BATTLE_HEIGHT: usize = 32;
const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BattleTile {
    Open,
    Wall,
}

pub struct BattleMap {
    pub tiles: Vec<BattleTile>,
}

impl BattleMap {
    fn new() -> Self {
        Self {
            tiles: vec![BattleTile::Open; BATTLE_WIDTH * BATTLE_HEIGHT],
        }
    }

    pub fn from_region_map(player_pos: &WanderingPlayer, map: &RegionMap) -> Self {
        let mut result = Self::new();

        let mut center_x = player_pos.x as usize;
        let mut center_y = player_pos.y as usize;
        while center_x < 2 {
            center_x += 1;
        }
        while center_y < 2 {
            center_x += 1;
        }
        while center_x > map.size.0 as usize - 3 {
            center_x -= 1;
        }
        while center_y > map.size.1 as usize - 3 {
            center_y -= 1;
        }

        for y in center_y - 2..center_y + 2 {
            for x in center_x - 2..center_x + 2 {
                let tile_map = region_tile_to_battle_map(map, x, y);
                if x > 0 && x < map.size.0 as usize && y > 0 && y < map.size.1 as usize {
                    tiles_to_battle_map(
                        x,
                        y,
                        center_x - 2,
                        center_y - 2,
                        &tile_map,
                        &mut result.tiles,
                    );
                } else {
                    println!("Tile out of world map: {},{}", x, y);
                }
            }
        }

        result
    }
}

fn tiles_to_battle_map(
    battle_x: usize,
    battle_y: usize,
    left_x: usize,
    top_y: usize,
    chunk: &[BattleTile],
    map: &mut [BattleTile],
) {
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let chunk_idx = (TILE_WIDTH * y) + x;
            let bx = ((battle_x - left_x) * TILE_WIDTH) + x;
            let by = ((battle_y - top_y) * TILE_HEIGHT) + y;
            let battle_idx = (by * BATTLE_WIDTH) + bx;
            if battle_idx < map.len() - 1 {
                map[battle_idx] = chunk[chunk_idx];
            } else {
                println!("Out of bounds: {}, {}", bx, by);
            }
        }
    }
}

fn exit_tiles(dir: Direction) -> Vec<usize> {
    match dir {
        Direction::North => (0..TILE_WIDTH).collect(),
        Direction::South => {
            let first = TILE_WIDTH * (TILE_HEIGHT - 1);
            (first..first + TILE_WIDTH).collect()
            //Vec::new()
        }
        Direction::West => {
            let mut result = Vec::with_capacity(TILE_HEIGHT);
            for y in 0..TILE_HEIGHT {
                result.push(y * TILE_WIDTH);
            }
            result
        }
        Direction::East => {
            let mut result = Vec::with_capacity(TILE_HEIGHT);
            for y in 0..TILE_HEIGHT {
                result.push((y * TILE_WIDTH) + (TILE_WIDTH - 1));
            }
            result
        }
    }
}

fn fill_exit_tiles(
    boundary_tiles: &[usize],
    exit_type: RegionBoundaryType,
    map: &mut [BattleTile],
) {
    match exit_type {
        RegionBoundaryType::None => {
            boundary_tiles
                .iter()
                .for_each(|idx| map[*idx] = BattleTile::Open);
        }
        RegionBoundaryType::Wall => {
            boundary_tiles
                .iter()
                .for_each(|idx| map[*idx] = BattleTile::Wall);
        }
        RegionBoundaryType::Opening => {
            let len = boundary_tiles.len();
            boundary_tiles.iter().enumerate().for_each(|(i, idx)| {
                if i < len / 3 || i > (len / 3) * 2 {
                    map[*idx] = BattleTile::Wall;
                } else {
                    map[*idx] = BattleTile::Open;
                }
            });
        }
    }
}

fn region_tile_to_battle_map(map: &RegionMap, x: usize, y: usize) -> Vec<BattleTile> {
    let tile_idx = (y * map.size.0 as usize) + x;
    let tile = &map.tiles[tile_idx];
    let mut result = vec![BattleTile::Open; TILE_HEIGHT * TILE_WIDTH];

    if tile.tile_type == RegionTileType::Solid {
        result.iter_mut().for_each(|t| *t = BattleTile::Wall);
    }

    fill_exit_tiles(
        &exit_tiles(Direction::North),
        tile.boundaries[Direction::North.to_exit_index()].0,
        &mut result,
    );
    fill_exit_tiles(
        &exit_tiles(Direction::South),
        tile.boundaries[Direction::South.to_exit_index()].0,
        &mut result,
    );
    fill_exit_tiles(
        &exit_tiles(Direction::East),
        tile.boundaries[Direction::East.to_exit_index()].0,
        &mut result,
    );
    fill_exit_tiles(
        &exit_tiles(Direction::West),
        tile.boundaries[Direction::West.to_exit_index()].0,
        &mut result,
    );

    result
}
