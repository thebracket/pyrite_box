use serde::{Deserialize, Serialize};

use crate::game_states::player_movement::PlayerMoveRequest;

#[derive(Clone, Serialize, Deserialize)]
pub struct EventList {
    pub filename: String,
    pub events: Vec<GameEvent>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub tag: String,
    pub steps: Vec<GameEventStep>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameEventStep {
    LogText {
        text: String,
        color: Option<(u8, u8, u8)>,
    },
    ClearLog,
    PauseMs(u64),
    CallEvent(String),
    MovePlayer(PlayerMoveRequest, u64),
    ChangeMap{index: usize, x: u32, y: u32},
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EventPicker {
    LogText,
    ClearLog,
    PauseMs,
    CallEvent,
    MovePlayer,
}
