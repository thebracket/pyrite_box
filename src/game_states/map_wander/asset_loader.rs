use super::gamelog::GameLog;
use crate::{
    game_states::ModuleSelector, module::game_events::ScriptState,
    region::region_assets::RegionAssets, AppState,
};
use bevy::{asset::LoadState, prelude::*};
use bevy_egui::egui;
use bevy_egui::{egui::Pos2, EguiContext};

pub struct MaterialLoader {
    total: usize,
    remaining: Vec<HandleUntyped>,
}

pub fn start_asset_loader(
    mut commands: Commands,
    startup: Res<ModuleSelector>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut egui_context: ResMut<EguiContext>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Create an empty ScriptState to hold the current scripting engine state.
    // Create an empty GameLog
    commands.insert_resource(ScriptState::new());
    commands.insert_resource(GameLog::new());

    // Select the module
    let module = startup.module.as_ref().unwrap().clone();

    // Determine player starting location
    let map_idx = module.starting_map_idx;

    // Initiate the asset loading process.
    // This is async, so we have to track it. If we don't wait for
    // everything to load, the game will have "popping" as things appear.
    let mut handles = Vec::new();
    let assets = RegionAssets::new(
        &mut materials,
        &mut meshes,
        &asset_server,
        &module,
        map_idx,
        &mut egui_context,
        &mut handles,
        &mut texture_atlases,
    );

    // Now we pack all of the materials we requested into a MaterialLoader
    // so we can track progress.
    let materials = MaterialLoader {
        total: handles.len(),
        remaining: handles,
    };
    commands.insert_resource(materials);
    commands.insert_resource(assets); // This moves the structure, do it last
}

pub fn asset_loader(
    egui_context: ResMut<EguiContext>,
    server: Res<AssetServer>,
    mut materials: ResMut<MaterialLoader>,
    mut state: ResMut<State<AppState>>,
) {
    materials.remaining.retain(|h| {
        let state = server.get_load_state(h.id);
        state != LoadState::Loaded
    });

    egui::Window::new("Loading Module")
        .resizable(false)
        .title_bar(true)
        .fixed_pos(Pos2::new(200.0, 100.0))
        .fixed_size(bevy_egui::egui::Vec2::new(800.0, 500.0))
        .show(egui_context.ctx(), |ui| {
            ui.label("Loading Module Assets, Please Wait...");
            ui.label(format!(
                "Remaining: {} of {}",
                materials.remaining.len(),
                materials.total
            ));
            ui.add(
                egui::ProgressBar::new(
                    (materials.total as f32 - materials.remaining.len() as f32)
                        / materials.total as f32,
                )
                .show_percentage(),
            );
        });

    if materials.remaining.is_empty() {
        // Switch state
        state
            .set(AppState::MapWander)
            .expect("Failed to change mode");
    }
}

pub fn finish_asset_loader(mut commands: Commands) {
    commands.remove_resource::<MaterialLoader>();
}
