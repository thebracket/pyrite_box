use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
mod game_states;
mod region;
use game_states::{*, gamelog::display_game_log};
mod module;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Loading,
    MainMenu,
    ModuleEditor,
    MapWander, // Test mode for the map
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Pyrite Box".to_string(),
            width: 1280.0,
            height: 1024.0,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_state(AppState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_fps.system())
        .add_startup_system(setup_ui.system())
        .add_system(fps_update_system.system())
        // Loading State
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(loading_screen.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Loading).with_system(resume_loading_screen.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit_loading.system()))
        // Main Menu State
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(main_menu.system()), //.with_system(texture_mode_system.system())
        )
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(resume_main_menu.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit_main_menu.system()))
        // Module Editor
        .add_system_set(
            SystemSet::on_update(AppState::ModuleEditor).with_system(module_editor.system()), //.with_system(texture_mode_system.system())
        )
        .add_system_set(
            SystemSet::on_enter(AppState::ModuleEditor).with_system(resume_module_editor.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::ModuleEditor).with_system(exit_module_editor.system()),
        )
        // Map Wander
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(map_wander.system()))
        .add_system_set(
            SystemSet::on_update(AppState::MapWander).with_system(map_wander_rebuild.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MapWander).with_system(display_game_log.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::MapWander).with_system(resume_map_wander.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MapWander).with_system(exit_map_wander.system()),
        )
        .run();
}
