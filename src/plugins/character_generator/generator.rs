use crate::{state::ChargenResource, AppState};
use bevy::prelude::*;
use bevy_egui::{egui, egui::Pos2, EguiContext};

pub fn start_chargen() {}

pub fn run_chargen(
    mut chargen: ResMut<ChargenResource>,
    egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
) {
    egui::Window::new("Character Information")
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(200.0, 100.0))
        .fixed_size(bevy_egui::egui::Vec2::new(800.0, 300.0))
        .show(egui_context.ctx(), |ui| {
            ui.label("Character Name");
            ui.text_edit_singleline(&mut chargen.character.name);
            if ui.button("Save").clicked() {
                chargen.character.save().unwrap();
            }

            if ui.button("Return to main menu").clicked() {
                state
                    .set(AppState::MainMenu)
                    .expect("Failed to change mode");
            }
        });
}

pub fn exit_chargen() {}
