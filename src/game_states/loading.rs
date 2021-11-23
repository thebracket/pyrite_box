use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, egui::Pos2, EguiContext};

pub struct LoadingResource {}

pub fn loading_screen(
    egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut res: ResMut<LoadingResource>,
) {
    egui::Window::new("Loading - Please Wait")
        .auto_sized()
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(500.0, 200.0))
        .show(egui_context.ctx(), |ui| {
            ui.label("Please wait");
        });
    state
        .set(AppState::MainMenu)
        .expect("Failed to change mode");
}

pub fn resume_loading_screen(mut commands: Commands) {
    commands.insert_resource(LoadingResource {})
}

pub fn exit_loading(mut commands: Commands) {
    commands.remove_resource::<LoadingResource>();
}
