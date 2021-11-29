use crate::region::region_map::map_editor::MapEditor;
use crate::region::Direction;
use crate::region::{
    region_assets::RegionAssets,
    region_map::{geometry::GEOMETRY_SIZE, RegionMap},
};
use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_egui::egui::Frame;
use bevy_egui::{
    egui::{Pos2, Window},
    EguiContext,
};

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
    map: RegionMap,
}

pub fn map_wander(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut WanderingPlayer>,
    mut move_set: QuerySet<(
        Query<(&WanderLight, &mut Transform)>,
        Query<(&WanderCamera, &mut Transform)>,
        Query<(Entity, &WanderGeometry)>,
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
                ui.label(format!(
                    "{}. X: {}, Y: {}, Facing: {:?}",
                    wander.map.name, wp.x, wp.y, wp.facing
                ));
            });

        Window::new("Map")
            .default_size(bevy_egui::egui::vec2(512.0, 512.0))
            .scroll(false)
            .show(egui_context.ctx(), |ui| {
                ui.text_edit_singleline(&mut wander.map.name);
                Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    let mut ed = MapEditor {
                        map: &mut wander.map,
                    };
                    ed.ui_content(ui)
                });
            });
    });
}

pub fn resume_map_wander(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = RegionMap::default();
    let (start_x, start_y, start_z) = map.starting_location;
    let assets = RegionAssets::new(&mut materials, &mut meshes, &map);
    for m in assets.meshes.iter() {
        // TODO: m.0 tells you what material to use
        commands
            .spawn_bundle(PbrBundle {
                mesh: m.1.clone(),
                material: assets.materials[m.0 as usize].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(MapWander {})
            .insert(WanderGeometry{});
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
    let perpsective = PerspectiveProjection {
        fov: 1.5708,
        aspect_ratio: 1280.0 / 1024.0,
        near: 0.1,
        far: 1000.0,
    };

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            perspective_projection: perpsective,
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
    commands.insert_resource(WanderResource { map });
}

pub fn exit_map_wander(mut commands: Commands, cleanup: Query<(Entity, &MapWander)>) {
    cleanup
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn());
}

pub fn map_wander_rebuild(
    geometry_query: Query<(Entity, &WanderGeometry)>,
    mut wander: ResMut<WanderResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut assets: ResMut<RegionAssets>,
) {
    if wander.map.needs_rebuild {
        wander.map.needs_rebuild = false;
        geometry_query.iter().for_each(|(e, ..)| {
            commands.entity(e).despawn();
        });
        for m in assets.materials.iter() {
            materials.remove(m.clone());
        }
        for m in assets.meshes.iter() {
            meshes.remove(m.1.clone());
        }
        *assets = RegionAssets::new(&mut materials, &mut meshes, &wander.map);
        for m in assets.meshes.iter() {
            // TODO: m.0 tells you what material to use
            commands
                .spawn_bundle(PbrBundle {
                    mesh: m.1.clone(),
                    material: assets.materials[m.0 as usize].clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                })
                .insert(MapWander {})
                .insert(WanderGeometry{});
        }
    }
}