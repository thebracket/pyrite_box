use serde::{Deserialize, Serialize};

use crate::game_states::gamelog::GameLog;

use super::Module;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub tag: String,
    pub steps: Vec<GameEventStep>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameEventStep {
    LogText(String),
    CallEvent(String),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EventPicker {
    LogText,
    CallEvent,
}

pub fn run_events(module: &Module, tag: &str, log: &mut GameLog) {
    if let Some(event) = module.events.iter().find(|e| e.tag.eq(tag)) {
        for step in event.steps.iter() {
            match &step {
                GameEventStep::LogText(text) => {
                    log.add_line(&text);
                }
                GameEventStep::CallEvent(tag) => {
                    run_events(module, tag, log);
                }
            }
        }
    }
}
