use std::collections::VecDeque;
use super::GameEventStep;
use crate::game_states::{gamelog::GameLog, WanderResource};
use bevy::prelude::*;

#[derive(Clone)]
pub struct TriggerEvent(pub String);

pub fn event_triggers(
    mut events: EventReader<TriggerEvent>,
    mut state: ResMut<ScriptState>,
) {
    for trigger in events.iter() {
        state.event_queue.push_front(trigger.clone());
    }
}

pub fn event_runner(
    mut wander: ResMut<WanderResource>,
    mut state: ResMut<ScriptState>,
    mut log : ResMut<GameLog>,
) {
    wander.allow_movement = false;
    let mut new_stack_entries = Vec::new();
    if let Some(stack_entry) = state.stack.pop() {
        if let Some(event) = wander.module.events.iter().find(|e| e.tag.eq(&stack_entry.tag)) {
            if stack_entry.line < event.steps.len() {
                // Put the next event into the stack
                new_stack_entries.push(ScriptPoint{
                    tag: stack_entry.tag.clone(),
                    line: stack_entry.line+1,
                });

                // Execute it
                match &event.steps[stack_entry.line] {
                    GameEventStep::LogText(text) => {
                        log.add_line(&text);
                    }
                    GameEventStep::CallEvent(tag) => {
                        // Add the jump to the stack
                        // The next event in this script is also in the stack, so it'll resume
                        // upon return.
                        new_stack_entries.push(ScriptPoint{
                            tag: tag.clone(), line: 0
                        })
                    }
                }
            } else {
                // We reached the end of the event without a jump
                return;
            }
        } else {
            // The tag didn't exist
            println!("Script error: {} not found", stack_entry.tag);
            return;
        }

        new_stack_entries.drain(..).for_each(|s| state.stack.push(s));
        return;
    }
    if let Some(new_event) = state.event_queue.pop_back() {
        state.stack.push(ScriptPoint{ tag : new_event.0, line: 0 });
        return;
    }

    // There weren't any events, so don't block input
    wander.allow_movement = true;
}

/// Represents the execution stack of scripts that run one step per
/// tick.
pub struct ScriptStack {
    pub stack : Vec<ScriptPoint>,
}

impl ScriptStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn pop(&mut self) -> Option<ScriptPoint> {
        self.stack.pop()
    }

    pub fn push(&mut self, event : ScriptPoint) {
        self.stack.push(event);
    }
}

/// Represents an execution point within a script
pub struct ScriptPoint {
    /// The event tag at which the script points
    pub tag: String,

    /// The current execution index
    pub line: usize,
}

/// Current scripting state
/// Intended to be a resource
pub struct ScriptState {
    pub event_queue : VecDeque<TriggerEvent>,
    pub stack: ScriptStack,
}

impl ScriptState {
    pub fn new() -> Self {
        Self {
            event_queue : VecDeque::new(),
            stack: ScriptStack::new(),
        }
    }
}