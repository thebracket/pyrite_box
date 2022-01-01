use super::{WanderCamera, WanderLight, WanderResource, WanderingPlayer};
use crate::{module::game_events::TriggerEvent, region::region_map::geometry::GEOMETRY_SIZE};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerMoveRequest {
    Forwards,
    Backwards,
    TurnLeft,
    TurnRight,
    ChangeMap { index: usize, x: u32, y: u32 },
}

pub fn player_move(
    mut events: EventReader<PlayerMoveRequest>,
    mut player_query: Query<&mut WanderingPlayer>,
    mut move_set: QuerySet<(
        Query<(&WanderLight, &mut Transform)>,
        Query<(&WanderCamera, &mut Transform)>,
    )>,
    mut wander: ResMut<WanderResource>,
    mut triggers: EventWriter<TriggerEvent>,
) {
    let mut moved = false;
    let map_idx = wander.map_idx;
    let mut previous_location = 0;
    player_query.iter_mut().for_each(|mut wp| {
        previous_location = (wander.module.maps[&map_idx].size.0 * wp.y as u32) + wp.x as u32;
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
                PlayerMoveRequest::ChangeMap { index, x, y } => {
                    wp.x = *x as i32;
                    wp.y = *y as i32;
                    wander.map_idx = *index;
                    moved = true;
                    wander.module.maps.get_mut(index).unwrap().needs_rebuild = true;
                }
            }
        }

        if moved {
            wp.x = wp.x.clamp(0, wander.module.maps[&map_idx].size.0 as i32);
            wp.y = wp.y.clamp(0, wander.module.maps[&map_idx].size.1 as i32);
            let new_location = (wander.module.maps[&map_idx].size.0 * wp.y as u32) + wp.x as u32;
            if let Some((direction, trigger)) =
                &wander.module.maps[&map_idx].tiles[previous_location as usize].exit_trigger
            {
                if wp.facing == *direction {
                    triggers.send(TriggerEvent(trigger.clone()));
                }
            }
            if let Some(trigger) =
                &wander.module.maps[&map_idx].tiles[new_location as usize].entry_trigger
            {
                triggers.send(TriggerEvent(trigger.clone()));
            }
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
