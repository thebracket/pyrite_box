use bevy::math::Vec3;
use bevy_egui::egui::Vec2;
use serde::{Deserialize, Serialize};

/// Represents a cardinal direction (North, South, East, West)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Calculate the result of rotating 90 degrees to the right
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Calculate the result of rotating 90 degrees to the left
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    /// Calculate the delta of moving forwards from this facing
    pub fn delta_forward(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    /// Calculate the delta of moving backwards from this facing
    pub fn delta_backward(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (-1, 0),
            Direction::South => (0, -1),
            Direction::West => (1, 0),
        }
    }

    /// Calculate a look_at vector for a camera at a given translation and
    /// this facing
    pub fn camera_look_at(&self, translation: &Vec3) -> Vec3 {
        match self {
            Direction::North => Vec3::new(translation.x, translation.y - 50.0, translation.z),
            Direction::South => Vec3::new(translation.x, translation.y + 50.0, translation.z),
            Direction::West => Vec3::new(translation.x + 50.0, translation.y, translation.z),
            Direction::East => Vec3::new(translation.x - 50.0, translation.y, translation.z),
        }
    }

    /// Translate a direction to a 0..4 direction index, used for arrays of directions.
    pub fn to_exit_index(&self) -> usize {
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

    /// Convert the facing to an aligned unit vec2
    pub fn to_direction_vec2(&self, length: f32) -> Vec2 {
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
