use bevy::prelude::*;
use bevy_egui::{
    egui::text::LayoutJob,
    egui::{Color32, Sense, Shape, TextFormat, Window},
    EguiContext,
};

pub const DEFAULT_TEXT_COLOR: Color32 = Color32::from_rgb(64, 255, 64);

pub struct GameLog {
    buffer: Vec<GameLogEntry>,
    pub blocking: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            blocking: false,
        }
    }

    pub fn add_line(&mut self, line: &str, color: Color32) {
        self.buffer.push(GameLogEntry {
            revealed: false,
            progress: 0,
            text: line.to_string(),
            color,
        });
    }
}

struct GameLogEntry {
    revealed: bool,
    progress: usize,
    text: String,
    color: Color32,
}

pub fn display_game_log(mut log: ResMut<GameLog>, egui_context: ResMut<EguiContext>) {
    let white = Color32::WHITE;

    Window::new("Log")
        .title_bar(true)
        .show(egui_context.ctx(), |ui| {
            let mut blocking = false;

            let mut job = LayoutJob::default();

            for e in log.buffer.iter_mut() {
                if e.revealed {
                    job.append(
                        e.text.as_str(),
                        0.0,
                        TextFormat {
                            style: bevy_egui::egui::TextStyle::Body,
                            color: e.color,
                            ..Default::default()
                        },
                    );
                    job.append(
                        "\n",
                        0.0,
                        TextFormat {
                            style: bevy_egui::egui::TextStyle::Body,
                            color: e.color,
                            ..Default::default()
                        },
                    );
                } else {
                    blocking = true;
                    if e.progress > 1 {
                        job.append(
                            &e.text.as_str()[..e.progress - 1],
                            0.0,
                            TextFormat {
                                style: bevy_egui::egui::TextStyle::Body,
                                color: e.color,
                                ..Default::default()
                            },
                        );
                        job.append(
                            &e.text.as_str()[e.progress - 1..e.progress],
                            0.0,
                            TextFormat {
                                style: bevy_egui::egui::TextStyle::Body,
                                color: white,
                                ..Default::default()
                            },
                        );
                    }
                    e.progress += 1;
                    if e.progress == e.text.len() {
                        e.revealed = true;
                    }
                }
            }

            job.wrap_width = ui.available_width();
            let galley = ui.fonts().layout_job(job);
            let (response, painter) = ui.allocate_painter(galley.size(), Sense::hover());
            painter.add(Shape::galley(response.rect.min, galley));

            log.blocking = blocking;
        });
}
