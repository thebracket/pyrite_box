use bevy::prelude::*;
use bevy_egui::{
    egui::text::LayoutJob,
    egui::{Color32, Sense, Shape, TextFormat, Window},
    EguiContext,
};
use std::time::Duration;

const MS_DELAY_LOG: u64 = 33;
pub const DEFAULT_TEXT_COLOR: Color32 = Color32::from_rgb(64, 255, 64);

pub struct GameLog {
    buffer: Vec<GameLogEntry>,
    pub blocking: bool,
    timer: Timer,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            blocking: false,
            timer: Timer::new(Duration::from_millis(MS_DELAY_LOG), false),
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

pub fn display_game_log(
    mut log: ResMut<GameLog>,
    egui_context: ResMut<EguiContext>,
    time: Res<Time>,
) {
    let white = Color32::WHITE;

    Window::new("Log")
        .title_bar(false)
        .resizable(false)
        .fixed_pos((0.0, 1024.0 - 220.0))
        .show(egui_context.ctx(), |ui| {
            let mut blocking = false;
            ui.set_height(220.0);
            ui.set_width(1280.0);

            let mut job = LayoutJob::default();

            log.timer.tick(time.delta());
            let timer_finished = log.timer.finished();
            let mut restart_timer = false;
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
                    if timer_finished {
                        restart_timer = true;
                        e.progress += 1;
                        if e.progress == e.text.len() {
                            e.revealed = true;
                        }
                    }
                }
            }

            if restart_timer {
                log.timer = Timer::new(Duration::from_millis(MS_DELAY_LOG), false);
            }

            job.wrap_width = ui.available_width();
            let galley = ui.fonts().layout_job(job);
            let (response, painter) = ui.allocate_painter(galley.size(), Sense::hover());
            painter.add(Shape::galley(response.rect.min, galley));

            log.blocking = blocking;
        });
}
