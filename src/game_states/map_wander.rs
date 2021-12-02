use crate::module::Module;
use crate::region::region_map::map_editor::{MapEditorSettings, MapEditor};
use crate::region::Direction;
use crate::region::{region_assets::RegionAssets, region_map::geometry::GEOMETRY_SIZE};
use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_egui::{
    egui::{Pos2, Window},
    EguiContext,
};

use super::ModuleSelector;

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
}

pub fn map_wander(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut WanderingPlayer>,
    mut move_set: QuerySet<(
        Query<(&WanderLight, &mut Transform)>,
        Query<(&WanderCamera, &mut Transform)>,
    )>,
    egui_context: ResMut<EguiContext>,
    mut wander: ResMut<WanderResource>,
) {
    player_query.iter_mut().for_each(|mut wp| {
        let mut moved = false;
        if keyboard_input.just_pressed(KeyCode::Right) {
            wp.facing = match wp.facing {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            };
            moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::Left) {
            wp.facing = match wp.facing {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            let (dx, dy) = match wp.facing {
                Direction::North => (0, -1),
                Direction::East => (1, 0),
                Direction::South => (0, 1),
                Direction::West => (-1, 0),
            };
            wp.x += dx;
            wp.y += dy;
            moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            let (dx, dy) = match wp.facing {
                Direction::North => (0, 1),
                Direction::East => (-1, 0),
                Direction::South => (0, -1),
                Direction::West => (1, 0),
            };
            wp.x += dx;
            wp.y += dy;
            moved = true;
        }

        if moved {
            move_set.q0_mut().iter_mut().for_each(|(_, mut trans)| {
                trans.translation.x = (wp.x as f32 * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
                trans.translation.y = (wp.y as f32 * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
            });
            move_set.q1_mut().iter_mut().for_each(|(_, mut trans)| {
                trans.translation.x = (wp.x as f32 * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
                trans.translation.y = (wp.y as f32 * GEOMETRY_SIZE) + (GEOMETRY_SIZE / 2.0);
                let target = match wp.facing {
                    Direction::North => Vec3::new(
                        trans.translation.x,
                        trans.translation.y - 50.0,
                        trans.translation.z,
                    ),
                    Direction::South => Vec3::new(
                        trans.translation.x,
                        trans.translation.y + 50.0,
                        trans.translation.z,
                    ),
                    Direction::East => Vec3::new(
                        trans.translation.x + 50.0,
                        trans.translation.y,
                        trans.translation.z,
                    ),
                    Direction::West => Vec3::new(
                        trans.translation.x - 50.0,
                        trans.translation.y,
                        trans.translation.z,
                    ),
                };
                trans.look_at(target, Vec3::new(0.0, 0.0, 1.0));
            });
        }

        Window::new("Navigation")
            .auto_sized()
            .resizable(false)
            .title_bar(false)
            .fixed_pos(Pos2::new(500.0, 100.0))
            .show(egui_context.ctx(), |ui| {
                let map_idx = wander.map_idx;
                ui.label(format!(
                    "{}. X: {}, Y: {}, Facing: {:?}",
                    wander.module.maps[&map_idx].name, wp.x, wp.y, wp.facing
                ));
            });

        let map_idx = wander.map_idx;
        let mut settings = wander.editor_settings.clone();
        MapEditor::render_in_module(egui_context.ctx(), &mut settings, &mut wander.module, map_idx);
        wander.editor_settings = settings;
    });
}

pub fn resume_map_wander(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    startup: Res<ModuleSelector>,
) {
    let module = Module::load(startup.filename.as_ref().unwrap());
    let map_idx = 0; // TODO: Change to starting map
    let (start_x, start_y, start_z) = {
        let (sx, sy) = module.maps[&map_idx].starting_location;
        let (x, y) = module.maps[&map_idx].tile_location(sx as f32, sy as f32);
        (
            x * GEOMETRY_SIZE,
            y * GEOMETRY_SIZE,
            0.5,
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
                Vec3::new(start_x, start_y - 50.0, start_z),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            ..Default::default()
        })
        .insert(MapWander {})
        .insert(WanderCamera {});

    // Wander player
    commands
        .spawn()
        .insert(WanderingPlayer {
            x: (start_x / GEOMETRY_SIZE) as i32,
            y: (start_y / GEOMETRY_SIZE) as i32,
            facing: Direction::North,
        })
        .insert(MapWander {});

    // Resource
    commands.insert_resource(WanderResource {
        map_idx, // TODO: Change to starting map from module
        module,
        editor_settings: MapEditorSettings::default(),
    });
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
    }
}