use crate::{
    module::Module,
    modules::{list_available_modules, ModuleHeader},
    AppState,
};
use bevy::{app::Events, prelude::*};
use bevy_egui::{
    egui::Pos2,
    egui::{self, Color32},
    EguiContext,
};

use super::CharacterHeader;

#[derive(Component)]
pub struct MainMenuUi;

pub struct AvailableModules {
    modules: Vec<ModuleHeader>,
    characters: Vec<CharacterHeader>,
}

pub struct ModuleSelector {
    pub module: Option<Module>,
    pub party: Vec<usize>,
}

pub fn main_menu(
    egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<State<AppState>>,
    available_modules: Res<AvailableModules>,
    mut selected_module: ResMut<ModuleSelector>,
) {
    egui::Window::new("Welcome to Pyrite Box")
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(200.0, 100.0))
        .fixed_size(bevy_egui::egui::Vec2::new(800.0, 300.0))
        .show(egui_context.ctx(), |ui| {
            ui.separator();

            for module in available_modules.modules.iter() {
                ui.heading(&module.name);
                ui.colored_label(Color32::LIGHT_GREEN, &module.description);
                ui.colored_label(Color32::GREEN, format!("Author: {}", &module.author));
                ui.horizontal(|ui| {
                    if selected_module.party.is_empty() && ui.button("Play").clicked() {
                        selected_module.module = Some(
                            crate::modules::load_module(module.filename.as_ref().unwrap()).unwrap(),
                        );
                        state
                            .set(AppState::MapWanderLoader)
                            .expect("Failed to change mode");
                    }
                    if ui.button("Edit").clicked() {
                        selected_module.module = Some(
                            crate::modules::load_module(module.filename.as_ref().unwrap()).unwrap(),
                        );
                        state
                            .set(AppState::ModuleEditor)
                            .expect("Failed to change mode");
                    }
                });
                ui.separator();
            }

            ui.heading("Other Options");
            ui.horizontal(|ui| {
                if ui.button("New Module").clicked() {
                    state
                        .set(AppState::ModuleEditor)
                        .expect("Failed to change mode");
                }

                // Quit game option
                if ui.button("Quit Program").clicked() {
                    app_exit_events.send(bevy::app::AppExit);
                }
            });
        });

    egui::Window::new("Assemble Your Party")
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(200.0, 400.0))
        .fixed_size(bevy_egui::egui::Vec2::new(800.0, 200.0))
        .show(egui_context.ctx(), |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.colored_label(Color32::WHITE, "Your Party");
                    if selected_module.party.is_empty() {
                        ui.colored_label(Color32::RED, "Please add 1-6 characters to your party.");
                    } else {
                        // List party
                    }
                });
                ui.vertical(|ui| {
                    ui.colored_label(Color32::WHITE, "Available Characters");
                    if available_modules.characters.is_empty() {
                        ui.colored_label(Color32::RED, "There are no available characters.");
                    }

                    /*
                    ui.button("<");
                    ui.button(">");
                    ui.button("Create New Character");
                    ui.button("Delete Character");
                    */
                });
            });
        });
}

pub fn resume_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainMenuUi {});
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/pyrite.png"),
            ..Default::default()
        })
        .insert(MainMenuUi {});

    // Find available modules
    commands.insert_resource(AvailableModules {
        modules: list_available_modules(),
        characters: CharacterHeader::scan_available(),
    });
    commands.insert_resource(ModuleSelector {
        module: None,
        party: Vec::new(),
    });
}

pub fn exit_main_menu(mut commands: Commands, cleanup: Query<(Entity, &MainMenuUi)>) {
    cleanup
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn());

    commands.remove_resource::<AvailableModules>();
}
