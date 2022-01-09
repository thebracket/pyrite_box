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
    // Obtain the player's map location
    let player_pos = player_query.iter().nth(0).unwrap();
    let current_map_idx = wander.map_idx;
    let current_map = &wander.module.maps[&current_map_idx];
    let battle_map = BattleMap::from_region_map(player_pos, current_map);

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(BattleComponent {});
    let mut i = 0;
    for y in 0..BATTLE_HEIGHT {
        for x in 0..BATTLE_WIDTH {
            let sprite = {
                if battle_map.tiles[i] == BattleTile::Open {
                    0
                } else {
                    1
                }
            };
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: assets.battle_tile_atlas.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 - BATTLE_WIDTH as f32 / 2.0) * 32.0,
                        ((y as f32 - BATTLE_HEIGHT as f32 / 2.0) * 32.0) + 16.0,
                        1.0,
                    ),
                    sprite: TextureAtlasSprite {
                        index: sprite,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BattleComponent {});
            i += 1;
        }
    }
}
