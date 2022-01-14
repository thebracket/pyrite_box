use serde::{Deserialize, Serialize};

use crate::plugins::{PlayerMoveRequest, SpriteRequest};

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
pub struct InputChoice {
    pub branch: String,
    pub message: String,
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
    InputBranch {
        title: String,
        message: String,
        portrait: Option<String>,
        options: Vec<InputChoice>,
    },
    ChangeMap {
        index: usize,
        x: u32,
        y: u32,
    },
    Sprite(SpriteRequest),
    Battle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EventPicker {
    LogText,
    ClearLog,
    PauseMs,
    CallEvent,
    MovePlayer,
}
