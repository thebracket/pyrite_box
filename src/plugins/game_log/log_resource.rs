use bevy::{core::Timer, prelude::Color};
use std::time::Duration;

pub const LOG_DELAY_MS: u64 = 10;

pub struct GameLogEntry {
    pub revealed: bool,
    pub progress: usize,
    pub text: String,
    pub color: Color,
    pub text_part: usize,
}

pub struct GameLog {
    pub buffer: Vec<GameLogEntry>,
    pub blocking: bool,
    pub timer: Timer,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            blocking: false,
            timer: Timer::new(Duration::from_millis(LOG_DELAY_MS), false),
        }
    }

    pub fn add_line(&mut self, line: &str, color: Color, part_no: usize) {
        self.buffer.push(GameLogEntry {
            revealed: false,
            progress: 0,
            text: line.to_string(),
            color,
            text_part: part_no,
        });
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}
