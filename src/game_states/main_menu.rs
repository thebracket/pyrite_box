use crate::AppState;
use bevy::{app::Events, prelude::*};
use bevy_egui::{egui, egui::Pos2, EguiContext};

pub fn main_menu(
    egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<State<AppState>>,
) {
    egui::Window::new("Hello")
        .auto_sized()
        .resizable(false)
        .title_bar(false)
        .fixed_pos(Pos2::new(500.0, 200.0))
        .show(egui_context.ctx(), |ui| {
            // Quit game option
            if ui.button("Quit").clicked() {
                app_exit_events.send(bevy::app::AppExit);
            }
        });
}

pub fn resume_main_menu(mut commands: Commands) {}

pub fn setup_main_menu(mut commands: Commands) {}

pub fn exit_main_menu(mut commands: Commands) {}
