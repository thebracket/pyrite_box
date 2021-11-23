use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use region::{region_assets::RegionAssets, region_map::RegionMap};
mod game_states;
mod region;
use game_states::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Loading,
    MainMenu,
}

fn main() {
    App::build()
        .add_state(AppState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(setup_fps.system())
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
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
    let map = RegionMap::default();
    let (start_x, start_y, start_z) = map.starting_location;
    let assets = RegionAssets::new(&mut materials, &mut meshes, &map);
    //commands.insert_resource(assets);
    for m in assets.meshes.iter() {
        commands.spawn_bundle(PbrBundle {
            mesh: m.clone(),
            material: assets.green.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
    }

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(start_x, start_y, start_z),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(start_x, start_y, start_z + 20.0)
            .looking_at(Vec3::new(start_x, start_y + 50.0, start_z), Vec3::Z),
        ..Default::default()
    });
    */
}
