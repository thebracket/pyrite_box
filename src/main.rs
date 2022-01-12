use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiPlugin;
mod game_states;
mod region;
use game_states::{
    asset_loader::*,
    gamelog::display_game_log,
    player_movement::{player_move, MoveOccurred, PlayerMoveRequest},
    sprites::{billboarding, region_sprites, SpriteRequest},
    *,
};
use module::game_events::{event_runner, event_triggers, TriggerEvent};
mod module;
mod modules;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Loading,
    MainMenu,
    ModuleEditor,
    MapWanderLoader, // Loading screen for the map module
    MapWander,       // Test mode for the map
    Battle,
    CharacterGeneration,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pyrite Box".to_string(),
            width: 1280.0,
            height: 1024.0,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_state(AppState::Loading)
        .add_event::<TriggerEvent>()
        .add_event::<PlayerMoveRequest>()
        .add_event::<SpriteRequest>()
        .add_event::<MoveOccurred>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_fps)
        .add_startup_system(setup_ui)
        .add_system(fps_update_system)
        // Loading State
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_screen))
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(resume_loading_screen))
        .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit_loading))
        // Main Menu State
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(main_menu))
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(resume_main_menu))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit_main_menu))
        // Module Editor
        .add_system_set(
            SystemSet::on_update(AppState::ModuleEditor).with_system(module_editor), //.with_system(texture_mode_system)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::ModuleEditor).with_system(resume_module_editor),
        )
        .add_system_set(SystemSet::on_exit(AppState::ModuleEditor).with_system(exit_module_editor))
        // Map Wander Loader
        .add_system_set(
            SystemSet::on_enter(AppState::MapWanderLoader).with_system(start_asset_loader),
        )
        .add_system_set(SystemSet::on_update(AppState::MapWanderLoader).with_system(asset_loader))
        .add_system_set(
            SystemSet::on_exit(AppState::MapWanderLoader).with_system(finish_asset_loader),
        )
        // Map Wander
        .add_system_set(SystemSet::on_enter(AppState::MapWander).with_system(resume_map_wander))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(map_wander))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(map_wander_rebuild))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(display_game_log))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(event_triggers))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(event_runner))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(player_move))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(region_sprites))
        .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(billboarding))
        .add_system_set(SystemSet::on_exit(AppState::MapWander).with_system(exit_map_wander))
        // Battle Mode
        .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(start_battle))
        .run();
}
