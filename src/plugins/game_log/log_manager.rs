use std::time::Duration;

use crate::plugins::UiFonts;
use bevy::prelude::*;

use super::{GameLog, LogMessage, LOG_DELAY_MS};

#[derive(Component)]
pub struct GameLogPanel;

pub fn update_game_log(
    mut log: ResMut<GameLog>,
    mut messages: EventReader<LogMessage>,
    mut log_finder: Query<(&GameLogPanel, &mut Text)>,
    font: Res<UiFonts>,
) {
    for msg in messages.iter() {
        match msg {
            LogMessage::Clear => {
                log.clear();
                log_finder.iter_mut().for_each(|(_, mut le)| {
                    le.sections.clear();
                });
            }
            LogMessage::AddLine { line, color } => {
                log_finder.iter_mut().for_each(|(_, mut le)| {
                    let part_no = le.sections.len();
                    log.add_line(line, *color, part_no);
                    le.sections.push(TextSection {
                        value: line[0..1].to_string(),
                        style: TextStyle {
                            font: font.game_font.clone(),
                            font_size: 20.0,
                            color: *color,
                        },
                    });
                    le.sections.push(TextSection {
                        value: "\n".to_string(),
                        style: TextStyle {
                            font: font.game_font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    });
                });
            }
        }
    }
}

pub fn animate_game_log(
    mut log: ResMut<GameLog>,
    mut log_finder: Query<(&GameLogPanel, &mut Text)>,
    time: Res<Time>,
) {
    log.timer.tick(time.delta());
    let timer_finished = log.timer.finished();
    let mut restart_timer = false;
    let mut blocking = false;
    log_finder.iter_mut().for_each(|(_, mut le)| {
        for e in log.buffer.iter_mut() {
            if !e.revealed {
                blocking = true;
                if timer_finished {
                    restart_timer = true;
                    e.progress += 1;
                    le.sections[e.text_part].value = e.text[0..e.progress - 1].to_string();
                    le.sections[e.text_part + 1].value =
                        format!("{}\n", e.text[e.progress - 1..e.progress].to_string());
                    if e.progress == e.text.len() {
                        e.revealed = true;
                        le.sections[e.text_part].value = e.text.clone();
                        le.sections[e.text_part + 1].value = "\n".to_string();
                    }
                }
            }
        }
    });
    if restart_timer {
        log.timer = Timer::new(Duration::from_millis(LOG_DELAY_MS), false);
    }
    log.blocking = blocking;
}

pub fn create_log_panel(mut commands: Commands, font: Res<UiFonts>) {
    // log
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                border: Rect::all(Val::Px(2.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(0.0),
                    bottom: Val::Percent(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // text
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font.game_font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(GameLogPanel {});
        });
}
