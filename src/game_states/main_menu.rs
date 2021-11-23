use crate::AppState;
use bevy::{app::Events, prelude::*};
use bevy_egui::{egui, egui::Pos2, EguiContext};
use super::UiAssets;

pub struct MainMenuUi;

pub fn main_menu(
    egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<State<AppState>>,
) {
    egui::Window::new("Welcome to Pyrite Box")
        .auto_sized()
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(500.0, 100.0))
        .show(egui_context.ctx(), |ui| {
            // Quit game option
            if ui.button("Quit Program").clicked() {
                app_exit_events.send(bevy::app::AppExit);
            }
        });
}

pub fn resume_main_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainMenuUi {});
    commands
        .spawn_bundle(SpriteBundle{
            material: ui_assets.title_mat.clone(),
            ..Default::default()
        })
        .insert(MainMenuUi{});
}

pub fn exit_main_menu(
    mut commands: Commands,
    cleanup : Query<(Entity, &MainMenuUi)>,
) {
    cleanup.iter().for_each(|(e, _)| commands.entity(e).despawn());
}
