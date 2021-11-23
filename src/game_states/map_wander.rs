use crate::region::{region_assets::RegionAssets, region_map::RegionMap, GEOMETRY_SIZE};
use bevy::prelude::*;
use bevy_egui::{
    egui::{Pos2, Window},
    EguiContext,
};

pub struct MapWander {}
pub struct WanderCamera {}
pub struct WanderLight {}

pub struct WanderingPlayer {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
}

#[derive(Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub fn map_wander(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut WanderingPlayer>,
    mut move_set : QuerySet<(
        Query<(&WanderLight, &mut Transform)>,
        Query<(&WanderCamera, &mut Transform)>,
    )>,
    egui_context: ResMut<EguiContext>,
) {
    player_query.iter_mut().for_each(|mut wp| {
        let mut moved = false;
        if keyboard_input.just_pressed(KeyCode::Left) {
            wp.facing = match wp.facing {
                Direction::NORTH => Direction::WEST,
                Direction::EAST => Direction::NORTH,
                Direction::SOUTH => Direction::EAST,
                Direction::WEST => Direction::SOUTH,
            };
            moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            wp.facing = match wp.facing {
                Direction::NORTH => Direction::EAST,
                Direction::EAST => Direction::SOUTH,
                Direction::SOUTH => Direction::WEST,
                Direction::WEST => Direction::NORTH,
            };
            moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            let (dx,dy) = match wp.facing {
                Direction::NORTH => (0, -1),
                Direction::EAST => (1, 0),
                Direction::SOUTH => (0, 1),
                Direction::WEST => (-1, 0),
            };
            wp.x += dx;
            wp.y += dy;
            moved = true;
        }

        if moved {
            move_set.q0_mut().iter_mut().for_each(|(_, mut trans)| {
                trans.translation.x = wp.x as f32 * GEOMETRY_SIZE;
                trans.translation.y = wp.y as f32 * GEOMETRY_SIZE;
            });
            move_set.q1_mut().iter_mut().for_each(|(_, mut trans)| {
                trans.translation.x = wp.x as f32 * GEOMETRY_SIZE;
                trans.translation.y = wp.y as f32 * GEOMETRY_SIZE;
                let target = match wp.facing {
                    Direction::NORTH => Vec3::new(trans.translation.x, trans.translation.y - 50.0, trans.translation.z),
                    Direction::SOUTH => Vec3::new(trans.translation.x, trans.translation.y + 50.0, trans.translation.z),
                    Direction::EAST => Vec3::new(trans.translation.x + 50.0, trans.translation.y, trans.translation.z),
                    Direction::WEST => Vec3::new(trans.translation.x - 50.0, trans.translation.y, trans.translation.z),
                };
                trans.look_at(target, Vec3::Z)
            });
        }

        Window::new("Navigation")
            .auto_sized()
            .resizable(false)
            .title_bar(false)
            .fixed_pos(Pos2::new(500.0, 100.0))
            .show(egui_context.ctx(), |ui| {
                ui.label(format!("X: {}, Y: {}, Facing: {:?}", wp.x, wp.y, wp.facing));
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
    //commands.insert_resource(assets);
    for m in assets.meshes.iter() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: m.clone(),
                material: assets.green.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(MapWander {});
    }

    // light
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(start_x, start_y, start_z),
            ..Default::default()
        })
        .insert(MapWander {})
        .insert(WanderLight{});

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(start_x, start_y, start_z)
                .looking_at(Vec3::new(start_x, start_y - 50.0, start_z), Vec3::Z),
            ..Default::default()
        })
        .insert(MapWander {})
        .insert(WanderCamera{});

    // Wander player
    commands
        .spawn()
        .insert(WanderingPlayer {
            x: (start_x / GEOMETRY_SIZE) as i32,
            y: (start_y / GEOMETRY_SIZE) as i32,
            facing: Direction::NORTH,
        })
        .insert(MapWander {});
}

pub fn exit_map_wander(mut commands: Commands, cleanup: Query<(Entity, &MapWander)>) {
    cleanup
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn());
}
