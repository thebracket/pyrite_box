use crate::utils::Direction;
use bevy::prelude::Component;

#[derive(Component)]
pub struct MapWander {}
#[derive(Component)]
pub struct WanderGeometry {}
#[derive(Component)]
pub struct WanderCamera {}
#[derive(Component)]
pub struct WanderLight {}

#[derive(Component, Debug)]
pub struct WanderingPlayer {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
}
