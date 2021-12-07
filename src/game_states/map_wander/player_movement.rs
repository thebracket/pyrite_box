use super::{WanderCamera, WanderLight, WanderResource, WanderingPlayer};
use crate::region::region_map::geometry::GEOMETRY_SIZE;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerMoveRequest {
    Forwards,
    Backwards,
    TurnLeft,
    TurnRight,
}

pub fn player_move(
    mut events: EventReader<PlayerMoveRequest>,
    mut player_query: Query<&mut WanderingPlayer>,
    mut move_set: QuerySet<(
        Query<(&WanderLight, &mut Transform)>,
        Query<(&WanderCamera, &mut Transform)>,
    )>,
    wander: Res<WanderResource>,
) {
    let mut moved = false;
    player_query.iter_mut().for_each(|mut wp| {
        for event in events.iter() {
            match event {
                PlayerMoveRequest::TurnRight => {
                    wp.facing = wp.facing.turn_right();
                    moved = true;
                }
                PlayerMoveRequest::TurnLeft => {
                    wp.facing = wp.facing.turn_left();
                    moved = true;
                }
                PlayerMoveRequest::Backwards => {
                    let (dx, dy) = wp.facing.delta_backward();
                    wp.x += dx;
                    wp.y += dy;
                    moved = true;
                }
                PlayerMoveRequest::Forwards => {
                    let (dx, dy) = wp.facing.delta_forward();
                    wp.x += dx;
                    wp.y += dy;
                    moved = true;
                }
            }
        }

        if moved {
            let map_idx = wander.map_idx;
            let (x, y) = wander.module.maps[&map_idx].tile_location(wp.x as f32, wp.y as f32);
            move_set.q0_mut().iter_mut().for_each(|(_, mut trans)| {
                trans.translation.x = (x * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
                trans.translation.y = (y * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
            });
            move_set.q1_mut().iter_mut().for_each(|(_, mut trans)| {
                trans.translation.x = (x * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
                trans.translation.y = (y * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
                let target = wp.facing.camera_look_at(&trans.translation);
                trans.look_at(target, Vec3::new(0.0, 0.0, 1.0));
            });
        }
    });
}
