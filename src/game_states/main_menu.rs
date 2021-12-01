use super::UiAssets;
use crate::AppState;
use bevy::{app::Events, prelude::*};
use bevy_egui::{egui, egui::Pos2, EguiContext};
use std::fs;

pub struct MainMenuUi;

pub struct AvailableModules {
    modules: Vec<String>,
}

pub struct ModuleSelector {
    pub filename: Option<String>,
}

pub fn main_menu(
    egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<State<AppState>>,
    available_modules: Res<AvailableModules>,
    mut selected_module: ResMut<ModuleSelector>,
) {
    egui::Window::new("Welcome to Pyrite Box")
        .auto_sized()
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(500.0, 100.0))
        .show(egui_context.ctx(), |ui| {
            ui.label(
                "Bracket's just-for-fun, kinda like the Gold Box games from SSI, test engine.",
            );

            if ui.button("New Module").clicked() {
                state
                    .set(AppState::ModuleEditor)
                    .expect("Failed to change mode");
            }

            for module in available_modules.modules.iter() {
                if ui.button(format!("Edit: {}", module)).clicked() {
                    selected_module.filename = Some(module.clone());
                    state
                        .set(AppState::ModuleEditor)
                        .expect("Failed to change mode");
                }
                if ui.button(format!("Launch: {}", module)).clicked() {
                    selected_module.filename = Some(module.clone());
                    state
                        .set(AppState::MapWander)
                        .expect("Failed to change mode");
                }
            }

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
        .spawn_bundle(SpriteBundle {
            material: ui_assets.title_mat.clone(),
            ..Default::default()
        })
        .insert(MainMenuUi {});

    // Find available modules
    let mut modules = Vec::new();
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        if let Ok(path) = path {
            if let Some(extension) = path.path().extension() {
                if extension == "pyr" {
                    modules.push(path.path().to_str().unwrap().to_string());
                }
            }
        }
    }
    commands.insert_resource(AvailableModules { modules });
    commands.insert_resource(ModuleSelector { filename: None });
}

pub fn exit_main_menu(mut commands: Commands, cleanup: Query<(Entity, &MainMenuUi)>) {
    cleanup
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn());

    commands.remove_resource::<AvailableModules>();
}
