use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContext};

use super::WanderResource;

pub struct GameLog {
    buffer: Vec<(bool, usize, String)>,
}

impl GameLog {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn add_line(&mut self, line: &str) {
        self.buffer.push((false, 0, line.to_string()));
    }
}

pub fn display_game_log(
    mut log: ResMut<GameLog>, 
    egui_context: ResMut<EguiContext>,
    mut wander: ResMut<WanderResource>,
) {
    Window::new("Log")
        .title_bar(true)
        .show(egui_context.ctx(), |ui| {
            for (revealed, progress, line) in log.buffer.iter_mut() {
                if *revealed {
                    ui.label(line.as_str());
                } else {
                    wander.allow_movement = false;
                    ui.label(
                        &line.as_str()[..*progress]
                    );
                    *progress += 1;
                    if *progress == line.len() {
                        *revealed = true;
                    }
                    return; // Stop displaying because we don't want to reveal future entries yet
                }
            }
            wander.allow_movement = true;
        });
}
