use crate::region::region_assets::RegionAssets;
use bevy::prelude::*;

use self::battle_map::{BattleMap, BattleTile, BATTLE_HEIGHT, BATTLE_WIDTH};
use super::{WanderResource, WanderingPlayer};
mod battle_map;

#[derive(Component)]
pub struct BattleComponent {}

pub fn start_battle(
    mut commands: Commands,
    wander: Res<WanderResource>,
    player_query: Query<&WanderingPlayer>,
    assets: Res<RegionAssets>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Obtain the player's map location
    let player_pos = player_query.iter().nth(0).unwrap();
    let current_map_idx = wander.map_idx;
    let current_map = &wander.module.maps[&current_map_idx];
    let battle_map = BattleMap::new()
        .setup_region_coordinates(player_pos)
        .build_from_region(current_map);
    battle_map.spawn_map_tiles(&mut commands, &assets);
}
