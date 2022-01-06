use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::region::{region_assets::RegionAssets, region_map::geometry::GEOMETRY_SIZE};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpriteRequest {
    Spawn {
        id: String,
        position: (u32, u32),
        image: String,
    },
    Remove {
        id: String,
    },
    Move {
        id: String,
        position: (u32, u32),
    },
}

#[derive(Component)]
pub struct RegionSprite(pub String);

pub fn region_sprites(
    mut events: EventReader<SpriteRequest>,
    mut commands: Commands,
    assets: Res<RegionAssets>,
    mut move_query: Query<(Entity, &RegionSprite, &mut Transform)>,
) {
    for event in events.iter() {
        match event {
            SpriteRequest::Spawn {
                id,
                position,
                image,
            } => {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: assets.sprite_mesh.clone(),
                        material: assets.sprites[image].clone(),
                        transform: Transform::from_xyz(
                            0.0 - ((position.0 as f32) * GEOMETRY_SIZE),
                            (position.1 as f32 + 0.5) * GEOMETRY_SIZE,
                            GEOMETRY_SIZE / 2.0,
                        ),
                        ..Default::default()
                    })
                    .insert(RegionSprite(id.clone()));
            }
            SpriteRequest::Move { id, position } => {
                move_query.iter_mut().for_each(|(_, tag, mut pos)| {
                    if tag.0.as_str() == id.as_str() {
                        pos.translation.x = 0.0 - ((position.0 as f32) * GEOMETRY_SIZE);
                        pos.translation.y = (position.1 as f32 + 0.5) * GEOMETRY_SIZE;
                    }
                });
            }
            SpriteRequest::Remove { id } => {
                move_query.iter().for_each(|(e, tag, _)| {
                    if tag.0.as_str() == id.as_str() {
                        commands.entity(e).despawn();
                    }
                });
            }
        }
    }
}
