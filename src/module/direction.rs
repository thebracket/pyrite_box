use bevy::math::Vec3;
use bevy_egui::egui::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn delta_forward(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    pub fn delta_backward(self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (-1, 0),
            Direction::South => (0, -1),
            Direction::West => (1, 0),
        }
    }

    pub fn camera_look_at(self, translation: &Vec3) -> Vec3 {
        match self {
            Direction::North => Vec3::new(translation.x, translation.y - 50.0, translation.z),
            Direction::South => Vec3::new(translation.x, translation.y + 50.0, translation.z),
            Direction::West => Vec3::new(translation.x + 50.0, translation.y, translation.z),
            Direction::East => Vec3::new(translation.x - 50.0, translation.y, translation.z),
        }
    }

    pub fn to_exit_index(self) -> usize {
        const NORTH: usize = 0;
        const SOUTH: usize = 1;
        const EAST: usize = 2;
        const WEST: usize = 3;

        match self {
            Direction::North => NORTH,
            Direction::South => SOUTH,
            Direction::East => EAST,
            Direction::West => WEST,
        }
    }

    pub fn to_direction_vec2(self, length: f32) -> Vec2 {
        match self {
            Direction::North => Vec2 { x: 0.0, y: -length },
            Direction::South => Vec2 { x: 0.0, y: length },
            Direction::East => Vec2 { x: length, y: 0.0 },
            Direction::West => Vec2 { x: -length, y: 0.0 },
        }
    }
}

impl From<usize> for Direction {
    fn from(item: usize) -> Self {
        match item {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }
}
