use super::GameEventStep;
use crate::{
    plugins::{GameLog, PlayerMoveRequest, SpriteRequest, DEFAULT_TEXT_COLOR},
    state::{WanderInput, WanderResource},
    AppState,
};
use bevy::prelude::*;
use bevy_egui::egui::Color32;
use std::{collections::VecDeque, time::Duration};

#[derive(Clone)]
pub struct TriggerEvent(pub String);

pub fn event_triggers(mut events: EventReader<TriggerEvent>, mut state: ResMut<ScriptState>) {
    for trigger in events.iter() {
        state.event_queue.push_front(trigger.clone());
    }
}

pub fn event_runner(
    mut wander: ResMut<WanderResource>,
    mut state: ResMut<ScriptState>,
    mut log: ResMut<GameLog>,
    time: Res<Time>,
    mut move_request: EventWriter<PlayerMoveRequest>,
    mut sprite_request: EventWriter<SpriteRequest>,
    mut app_state: ResMut<State<AppState>>,
) {
    wander.allow_movement = false;

    // This is where we stop execution if something else has the system's attention.
    // For example, don't run the next script node until a log entry has finished
    // rendering.
    if log.blocking {
        return;
    }
    if let Some(wi) = &wander.script_input {
        if wi.blocked {
            return;
        }
    }
    if let Some(timer) = &mut state.blocking_delay {
        timer.tick(time.delta());
        if !timer.finished() {
            return;
        }
    }

    // Walk the stack
    let mut new_stack_entries = Vec::new();
    let mut new_timer = None;
    let mut clear_script_input = false;
    if let Some(stack_entry) = state.stack.pop() {
        if let Some(event) = wander
            .module
            .events
            .events
            .iter()
            .find(|e| e.tag.eq(&stack_entry.tag))
        {
            if let Some(wi) = &wander.script_input {
                if let Some(idx) = wi.result {
                    let tag = &wi.options[idx].branch;
                    new_stack_entries.push(ScriptPoint {
                        tag: tag.clone(),
                        line: 0,
                    });
                    clear_script_input = true;
                }
            }

            if stack_entry.line < event.steps.len() {
                // Put the next event into the stack
                new_stack_entries.push(ScriptPoint {
                    tag: stack_entry.tag.clone(),
                    line: stack_entry.line + 1,
                });

                // Execute it
                match &event.steps[stack_entry.line] {
                    GameEventStep::LogText { text, color } => {
                        if let Some(color) = color {
                            log.add_line(text, Color32::from_rgb(color.0, color.1, color.2));
                        } else {
                            log.add_line(text, DEFAULT_TEXT_COLOR);
                        }
                    }
                    GameEventStep::ClearLog => {
                        log.clear();
                    }
                    GameEventStep::PauseMs(ms) => {
                        new_timer = Some(Timer::new(Duration::from_millis(*ms), false));
                    }
                    GameEventStep::MovePlayer(mv, delay) => {
                        move_request.send(*mv);
                        new_timer = Some(Timer::new(Duration::from_millis(*delay), false));
                    }
                    GameEventStep::ChangeMap { index, x, y } => {
                        move_request.send(PlayerMoveRequest::ChangeMap {
                            index: *index,
                            x: *x,
                            y: *y,
                        });
                    }
                    GameEventStep::CallEvent(tag) => {
                        // Add the jump to the stack
                        // The next event in this script is also in the stack, so it'll resume
                        // upon return.
                        new_stack_entries.push(ScriptPoint {
                            tag: tag.clone(),
                            line: 0,
                        })
                    }
                    GameEventStep::InputBranch {
                        message,
                        title,
                        portrait,
                        options,
                    } => {
                        wander.script_input = Some(WanderInput {
                            title: title.clone(),
                            message: message.clone(),
                            blocked: true,
                            options: options.clone(),
                            result: None,
                            portrait: portrait.clone(),
                        })
                    }
                    GameEventStep::Sprite(s) => {
                        sprite_request.send(s.clone());
                    }
                    GameEventStep::Battle => {
                        app_state
                            .set(AppState::Battle)
                            .expect("Failed to change mode");
                    }
                }
            } else {
                // We reached the end of the event without a jump
                //return;
            }
        } else {
            // The tag didn't exist
            println!("Script error: {} not found", stack_entry.tag);
            return;
        }

        if clear_script_input {
            wander.script_input = None;
        }

        new_stack_entries
            .drain(..)
            .for_each(|s| state.stack.push(s));

        state.blocking_delay = new_timer;
        return;
    }

    // If we've got this far, then there isn't a script running.
    // Check to see if a new one has been requested.
    if let Some(new_event) = state.event_queue.pop_back() {
        state.stack.push(ScriptPoint {
            tag: new_event.0,
            line: 0,
        });
        return;
    }

    // There weren't any events, so don't block input
    wander.allow_movement = true;
}

/// Represents the execution stack of scripts that run one step per
/// tick.
struct ScriptStack {
    stack: Vec<ScriptPoint>,
}

impl ScriptStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<ScriptPoint> {
        self.stack.pop()
    }

    fn push(&mut self, event: ScriptPoint) {
        self.stack.push(event);
    }
}

/// Represents an execution point within a script
struct ScriptPoint {
    /// The event tag at which the script points
    tag: String,

    /// The current execution index
    line: usize,
}

/// Current scripting state
/// Intended to be a resource
pub struct ScriptState {
    event_queue: VecDeque<TriggerEvent>,
    stack: ScriptStack,
    blocking_delay: Option<Timer>,
}

impl ScriptState {
    pub fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
            stack: ScriptStack::new(),
            blocking_delay: None,
        }
    }
}
