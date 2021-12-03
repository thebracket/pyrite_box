use bevy_egui::egui::Pos2;
use crate::module::Direction;
use super::scaling::Scaling;

pub fn wall_line(direction: Direction, x: u32, y:u32, scale: &Scaling) -> [Pos2; 2] {
    match direction {
        Direction::North => north_wall_line(x, y, scale),
        Direction::South => south_wall_line(x, y, scale),
        Direction::East => east_wall_line(x, y, scale),
        Direction::West => west_wall_line(x, y, scale),
    }
}

pub fn wall_opening(direction: Direction, x: u32, y: u32, scale: &Scaling) -> [[Pos2; 2]; 2] {
    match direction {
        Direction::North => north_wall_opening(x, y, scale),
        Direction::South => south_wall_opening(x, y, scale),
        Direction::East => east_wall_opening(x, y, scale),
        Direction::West => west_wall_opening(x, y, scale),
    }
}

fn north_wall_line(x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        scale.to_screen
            * Pos2 {
                x: px + scale.x10,
                y: py + scale.y10,
            },
        scale.to_screen
            * Pos2 {
                x: px + scale.box_x - scale.x10,
                y: py + scale.y10,
            },
    ]
}

fn south_wall_line(x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        scale.to_screen
            * Pos2 {
                x: px + scale.x10,
                y: py + scale.box_y - scale.y10,
            },
        scale.to_screen
            * Pos2 {
                x: px + scale.box_x - scale.x10,
                y: py + scale.box_y - scale.y10,
            },
    ]
}

fn west_wall_line(x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        scale.to_screen
            * Pos2 {
                x: px + scale.x10,
                y: py + scale.y10,
            },
        scale.to_screen
            * Pos2 {
                x: px + scale.x10,
                y: py + scale.box_y - scale.y10,
            },
    ]
}

fn east_wall_line(x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        scale.to_screen
            * Pos2 {
                x: px + scale.box_x - scale.x10,
                y: py + scale.y10,
            },
        scale.to_screen
            * Pos2 {
                x: px + scale.box_x - scale.x10,
                y: py + scale.box_y - scale.y10,
            },
    ]
}

fn north_wall_opening(x: u32, y: u32, scale: &Scaling) -> [[Pos2; 2]; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + (scale.x10 * 8.0),
                    y: py + scale.y10,
                },
        ],
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - (scale.x10 * 8.0),
                    y: py + scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.y10,
                },
        ],
    ]
}

fn south_wall_opening(x: u32, y: u32, scale: &Scaling) -> [[Pos2; 2]; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + (scale.x10 * 8.0),
                    y: py + scale.box_y - scale.y10,
                },
        ],
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - (scale.x10 * 8.0),
                    y: py + scale.box_y - scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
        ],
    ]
}

fn west_wall_opening(x: u32, y: u32, scale: &Scaling) -> [[Pos2; 2]; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + (scale.y10 * 8.0),
                },
        ],
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.box_y - (scale.y10 * 8.0),
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
        ],
    ]
}

fn east_wall_opening(x: u32, y: u32, scale: &Scaling) -> [[Pos2; 2]; 2] {
    let px = x as f32 * scale.box_x;
    let py = y as f32 * scale.box_y;
    [
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + (scale.y10 * 8.0),
                },
        ],
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.box_y - (scale.y10 * 8.0),
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
        ],
    ]
}