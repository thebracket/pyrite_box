use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContext};

pub struct GameLog {
    buffer: Vec<(bool, usize, String)>,
    pub blocking: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            blocking: false,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.buffer.push((false, 0, line.to_string()));
    }
}

pub fn display_game_log(mut log: ResMut<GameLog>, egui_context: ResMut<EguiContext>) {
    Window::new("Log")
        .title_bar(true)
        .show(egui_context.ctx(), |ui| {
            let mut blocking = false;
            for (revealed, progress, line) in log.buffer.iter_mut() {
                if *revealed {
                    ui.label(line.as_str());
                } else {
                    blocking = true;
                    ui.label(&line.as_str()[..*progress]);
                    *progress += 1;
                    if *progress == line.len() {
                        *revealed = true;
                    }
                }
            }

            log.blocking = blocking;
        });
}
