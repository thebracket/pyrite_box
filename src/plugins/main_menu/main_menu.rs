use super::MainMenuUi;
use crate::{
    modules::load_module,
    plugins::CharacterHeader,
    state::ChargenResource,
    state::{AvailableModules, ModuleSelector},
    AppState,
};
use bevy::{app::Events, prelude::*};
use bevy_egui::{
    egui::Pos2,
    egui::{self, Color32},
    EguiContext,
};

pub fn main_menu(
    egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<State<AppState>>,
    available_modules: Res<AvailableModules>,
    mut selected_module: ResMut<ModuleSelector>,
    mut commands: Commands,
) {
    egui::Window::new("Welcome to Pyrite Box")
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(200.0, 100.0))
        .fixed_size(bevy_egui::egui::Vec2::new(800.0, 300.0))
        .show(egui_context.ctx(), |ui| {
            ui.separator();

            show_modules(&available_modules, ui, &mut selected_module, &mut state);

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
                show_party(ui, &mut selected_module);
                show_characters(
                    ui,
                    &available_modules,
                    &mut selected_module,
                    &mut commands,
                    &mut state,
                );
            });
        });
}

fn show_modules(
    available_modules: &AvailableModules,
    ui: &mut egui::Ui,
    selected_module: &mut ModuleSelector,
    state: &mut bevy::prelude::State<AppState>,
) {
    for module in available_modules.modules.iter() {
        ui.heading(&module.name);
        ui.colored_label(Color32::LIGHT_GREEN, &module.description);
        ui.colored_label(Color32::GREEN, format!("Author: {}", &module.author));
        ui.horizontal(|ui| {
            if !selected_module.party.is_empty() && ui.button("Play").clicked() {
                selected_module.module =
                    Some(load_module(module.filename.as_ref().unwrap()).unwrap());
                state
                    .set(AppState::ModuleAssetLoader)
                    .expect("Failed to change mode");
            }
            if ui.button("Edit").clicked() {
                selected_module.module =
                    Some(load_module(module.filename.as_ref().unwrap()).unwrap());
                state
                    .set(AppState::ModuleEditor)
                    .expect("Failed to change mode");
            }
        });
        ui.separator();
    }
}

fn show_party(ui: &mut egui::Ui, selected_module: &mut ModuleSelector) {
    ui.vertical(|ui| {
        ui.colored_label(Color32::WHITE, "Your Party");
        if selected_module.party.is_empty() {
            ui.colored_label(Color32::RED, "Please add 1-6 characters to your party.");
        } else {
            let mut to_remove = None;
            for (i, name) in selected_module.party.iter().enumerate() {
                ui.colored_label(Color32::LIGHT_GREEN, name);
                if ui.button("Remove").clicked() {
                    to_remove = Some(i);
                }
            }
            if let Some(i) = to_remove {
                selected_module.party.remove(i);
            }
        }
    });
}

pub fn show_characters(
    ui: &mut egui::Ui,
    available_modules: &AvailableModules,
    selected_module: &mut ModuleSelector,
    commands: &mut Commands,
    state: &mut bevy::prelude::State<AppState>,
) {
    ui.vertical(|ui| {
        ui.colored_label(Color32::WHITE, "Available Characters");
        let available_characters: Vec<CharacterHeader> = available_modules
            .characters
            .iter()
            .filter(|chr| {
                selected_module
                    .party
                    .iter()
                    .find(|c| **c == chr.name)
                    .is_none()
            })
            .map(|chr| chr.clone())
            .collect();

        // If no characters are available, warn - otherwise iterate the characters
        if available_characters.is_empty() {
            ui.colored_label(Color32::RED, "There are no available characters.");
        } else {
            for chr in available_characters.iter() {
                ui.horizontal_top(|ui| {
                    ui.colored_label(Color32::LIGHT_GREEN, &chr.name);
                    if selected_module.party.len() < 6 {
                        if ui.button("Add to Party").clicked() {
                            selected_module.party.push(chr.name.clone());
                        }
                    }
                    if ui.button("Edit").clicked() {
                        commands.insert_resource(ChargenResource::with_character(chr.clone()));
                        state
                            .set(AppState::CharacterGeneration)
                            .expect("Failed to change mode");
                    }
                });
            }
        }

        // Link to character creation
        if ui.button("Create New Character").clicked() {
            commands.insert_resource(ChargenResource {
                character: CharacterHeader::new(),
            });
            state
                .set(AppState::CharacterGeneration)
                .expect("Failed to change mode");
        }
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
    commands.insert_resource(AvailableModules::new());
    commands.insert_resource(ModuleSelector::new());
}

pub fn exit_main_menu(mut commands: Commands, cleanup: Query<(Entity, &MainMenuUi)>) {
    cleanup
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn());

    commands.remove_resource::<AvailableModules>();
}
