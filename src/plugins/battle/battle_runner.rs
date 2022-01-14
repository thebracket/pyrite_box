use super::battle_map::BattleMap;
use crate::{plugins::WanderingPlayer, region::region_assets::RegionAssets, state::WanderResource};
use bevy::prelude::*;

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
