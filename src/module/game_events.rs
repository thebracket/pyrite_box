use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub tag: String,
    pub steps: Vec<GameEventStep>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameEventStep {
    LogText(String),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EventPicker {
    LogText,
}
