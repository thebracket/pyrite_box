use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContext};

pub struct GameLog {
    buffer: Vec<String>,
}

impl GameLog {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn add_line(&mut self, line: &str) {
        self.buffer.push(line.to_string());
    }
}

pub fn display_game_log(log: Res<GameLog>, egui_context: ResMut<EguiContext>) {
    Window::new("Log")
        .title_bar(true)
        .show(egui_context.ctx(), |ui| {
            for line in log.buffer.iter() {
                ui.label(line);
            }
        });
}
