use self::gamelog::GameLog;
use self::player_movement::PlayerMoveRequest;
use super::ModuleSelector;
use crate::module::game_events::ScriptState;
use crate::module::game_events::TriggerEvent;
use crate::module::Direction;
use crate::module::Module;
use crate::region::region_map::map_editor::{MapEditor, MapEditorSettings};
use crate::region::{region_assets::RegionAssets, region_map::geometry::GEOMETRY_SIZE};
use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_egui::{
    egui::{Pos2, Window},
    EguiContext,
};
pub mod gamelog;
pub mod player_movement;

pub struct MapWander {}
pub struct WanderGeometry {}
pub struct WanderCamera {}
pub struct WanderLight {}

pub struct WanderingPlayer {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
}

pub struct WanderResource {
    pub module: Module,
    pub map_idx: usize,
    pub editor_settings: MapEditorSettings,
    pub show_editor: bool,
    pub allow_movement: bool,
}

pub fn map_wander(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&WanderingPlayer>,
    egui_context: ResMut<EguiContext>,
    mut wander: ResMut<WanderResource>,
    mut move_request: EventWriter<PlayerMoveRequest>,
) {
    player_query.iter().for_each(|wp| {
        if wander.allow_movement {
            if keyboard_input.just_pressed(KeyCode::Right) {
                move_request.send(PlayerMoveRequest::TurnRight);
            }
            if keyboard_input.just_pressed(KeyCode::Left) {
                move_request.send(PlayerMoveRequest::TurnLeft);
            }
            if keyboard_input.just_pressed(KeyCode::Up) {
                move_request.send(PlayerMoveRequest::Forwards);
            }
            if keyboard_input.just_pressed(KeyCode::Down) {
                move_request.send(PlayerMoveRequest::Backwards);
            }
        }
        if keyboard_input.just_pressed(KeyCode::E) {
            wander.show_editor = !wander.show_editor;
        }

        Window::new("Navigation")
            .auto_sized()
            .resizable(false)
            .title_bar(false)
            .fixed_pos(Pos2::new(500.0, 25.0))
            .show(egui_context.ctx(), |ui| {
                let map_idx = wander.map_idx;
                ui.label(format!(
                    "{}. X: {}, Y: {}, Facing: {:?}",
                    wander.module.maps[&map_idx].name, wp.x, wp.y, wp.facing
                ));
            });

        if wander.show_editor {
            let map_idx = wander.map_idx;
            let mut settings = wander.editor_settings.clone();
            settings.highlight_player = Some((wp.x, wp.y, wp.facing));
            MapEditor::render_in_module(
                egui_context.ctx(),
                &mut settings,
                &mut wander.module,
                map_idx,
            );
            wander.editor_settings = settings;
        }
    });
}

pub fn resume_map_wander(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    startup: Res<ModuleSelector>,
    mut triggers: EventWriter<TriggerEvent>,
) {
    commands.insert_resource(ScriptState::new());
    let module = startup.0.as_ref().unwrap().clone();
    let map_idx = module.starting_map_idx;
    let (start_x, start_y, start_z, facing, tile_x, tile_y) = {
        let (sx, sy, direction) = module.maps[&map_idx].starting_location;
        let (x, y) = module.maps[&map_idx].tile_location(sx as f32, sy as f32);
        (
            (x + 0.5) * GEOMETRY_SIZE,
            (y + 0.5) * GEOMETRY_SIZE,
            0.5 * GEOMETRY_SIZE,
            direction,
            sx,
            sy,
        )
    };
    let assets = RegionAssets::new(&mut materials, &mut meshes, &asset_server, &module, map_idx);
    for m in assets.meshes.iter() {
        // TODO: m.0 tells you what material to use
        commands
            .spawn_bundle(PbrBundle {
                mesh: m.1.clone(),
                material: assets.materials[&(m.0 as usize)].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(MapWander {})
            .insert(WanderGeometry {});
    }
    commands.insert_resource(assets);

    // light
    commands
        .spawn_bundle(LightBundle {
            light: Light {
                color: Color::rgb(1.0, 1.0, 1.0),
                depth: 0.1..100.0,
                fov: f32::to_radians(60.0),
                intensity: 200.0,
                range: 100.0,
            },
            transform: Transform::from_xyz(start_x, start_y, start_z),
            ..Default::default()
        })
        .insert(MapWander {})
        .insert(WanderLight {});

    // camera
    let perspective = PerspectiveProjection {
        fov: 1.5708,
        aspect_ratio: 1280.0 / 1024.0,
        near: 0.1,
        far: 1000.0,
    };

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            perspective_projection: perspective,
            transform: Transform::from_xyz(start_x, start_y, start_z).looking_at(
                facing.camera_look_at(&Vec3::new(start_x, start_y, start_z)),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            ..Default::default()
        })
        .insert(MapWander {})
        .insert(WanderCamera {});

    // Wander player - start by running module/map initialization events
    if !module.module_start_event.is_empty() {
        triggers.send(TriggerEvent(module.module_start_event.clone()));
    }
    if !module.maps[&map_idx].map_start_event.is_empty() {
        triggers.send(TriggerEvent(module.maps[&map_idx].map_start_event.clone()));
    }
    commands
        .spawn()
        .insert(WanderingPlayer {
            x: tile_x as i32,
            y: tile_y as i32,
            facing,
        })
        .insert(MapWander {});

    // Resource
    commands.insert_resource(WanderResource {
        map_idx, // TODO: Change to starting map from module
        module,
        editor_settings: MapEditorSettings::default(),
        show_editor: false,
        allow_movement: true,
    });

    commands.insert_resource(GameLog::new());
}

pub fn exit_map_wander(mut commands: Commands, cleanup: Query<(Entity, &MapWander)>) {
    cleanup
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn());
}

pub fn map_wander_rebuild(
    mut wander: ResMut<WanderResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut assets: ResMut<RegionAssets>,
    geometry_query: Query<(Entity, &WanderGeometry)>,
) {
    let map_idx = wander.map_idx;
    if wander.module.maps[&map_idx].needs_rebuild {
        wander.module.maps.get_mut(&map_idx).unwrap().needs_rebuild = false;

        geometry_query.iter().for_each(|(e, ..)| {
            commands.entity(e).despawn();
        });

        assets.rebuild_geometry(&mut meshes, &wander.module, map_idx);

        for m in assets.meshes.iter() {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: m.1.clone(),
                    material: assets.materials[&(m.0 as usize)].clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                })
                .insert(MapWander {})
                .insert(WanderGeometry {});
        }
    }
}
