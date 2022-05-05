use super::player_movement::PlayerMoveRequest;
use super::{MapWander, WanderCamera, WanderGeometry, WanderLight, WanderingPlayer};
use crate::modules::game_events::TriggerEvent;
use crate::modules::Module;
use crate::plugins::UiFonts;
use crate::region::region_map::map_editor::MapEditor;
use crate::region::{region_assets::RegionAssets, region_map::geometry::GEOMETRY_SIZE};
use crate::state::MapEditorSettings;
use crate::state::ModuleSelector;
use crate::state::WanderResource;
use crate::utils::Direction;
use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_egui::egui;
use bevy_egui::{
    egui::{Pos2, Window},
    EguiContext,
};

pub fn play_region(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&WanderingPlayer>,
    mut egui_context: ResMut<EguiContext>,
    mut wander: ResMut<WanderResource>,
    mut move_request: EventWriter<PlayerMoveRequest>,
    assets: Res<RegionAssets>,
) {
    if let Some(wi) = &mut wander.script_input {
        egui::Window::new(&wi.title)
            .title_bar(true)
            .fixed_pos(Pos2::new(200.0, 200.0))
            .auto_sized()
            .show(egui_context.ctx_mut(), |ui| {
                if let Some(portrait_name) = &wi.portrait {
                    if let Some(id) = assets.ui_images.get(portrait_name) {
                        ui.image(*id, egui::Vec2::new(300.0, 300.0));
                    }
                }
                ui.label(&wi.message);
                for (i, opt) in wi.options.iter().enumerate() {
                    if ui.button(&opt.message).clicked() {
                        wi.result = Some(i);
                        wi.blocked = false;
                    }
                }
            });
    }

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
            .show(egui_context.ctx_mut(), |ui| {
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
                egui_context.ctx_mut(),
                &mut settings,
                &mut wander.module,
                map_idx,
            );
            wander.editor_settings = settings;
        }
    });
}

fn get_starting_position(module: &Module, map_idx: usize) -> (f32, f32, f32, Direction, u32, u32) {
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
}

pub fn resume_play_region(
    mut commands: Commands,
    startup: Res<ModuleSelector>,
    mut triggers: EventWriter<TriggerEvent>,
    assets: Res<RegionAssets>,
    wander: Option<ResMut<WanderResource>>,
    font: Res<UiFonts>,
) {
    let module = startup.module.as_ref().unwrap().clone();
    let map_idx = module.starting_map_idx;

    // main viewport
    // Not rendered because the camera viewport isn't
    // associated with the UI viewport system.

    // Party
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(25.0), Val::Percent(75.0)),
            border: Rect::all(Val::Px(2.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent(75.0),
                bottom: Val::Percent(25.0),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::rgb(0.6, 0.1, 0.1).into(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                border: Rect::all(Val::Percent(2.0)),
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        }).with_children(|parent| {
        parent
            .spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Party Panel!",
                    TextStyle {
                        font: font.game_font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
    });

    // Spawn the meshes for the map
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

    if wander.is_some() {
        // We're resuming from another state
        // We need to generate the camera & light - the player already exists, so we
        // can query it for location information.
    } else {
        // New game
        let (start_x, start_y, start_z, facing, tile_x, tile_y) =
            get_starting_position(&module, map_idx);

        // Wander player - start by running module/map initialization events
        if !module.module_start_event.is_empty() {
            triggers.send(TriggerEvent(module.module_start_event.clone()));
        }
        if !module.maps[&map_idx].map_start_event.is_empty() {
            triggers.send(TriggerEvent(module.maps[&map_idx].map_start_event.clone()));
        }

        // Resource
        commands.insert_resource(WanderResource {
            map_idx,
            module,
            editor_settings: MapEditorSettings::default(),
            show_editor: false,
            allow_movement: true,
            script_input: None,
        });

        // Entity for the light and camera

        // light
        commands
            .spawn_bundle(PointLightBundle {
                point_light: PointLight {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    // depth: 0.1..100.0,
                    // fov: f32::to_radians(60.0),
                    intensity: 1600.0,
                    range: 100.0,
                    //radius: f32::to_radians(360.0),
                    shadows_enabled: false,
                    ..Default::default()
                },
                transform: Transform::from_xyz(start_x, start_y, start_z),
                ..Default::default()
            })
            .insert(MapWander {})
            .insert(WanderLight {});

        // camera
        let perspective = PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_2, //1.5708,
            aspect_ratio: 1280.0 / 1024.0,
            near: 0.1,
            far: 1000.0,
        };

        let camera = Camera {
            //name: Some("camera_3d".to_string()),
            near: perspective.near,
            far: perspective.far,
            /*viewport: Some(bevy::render::camera::Viewport {
                x: 0.0,
                w: 0.75,
                h: 0.75,
                ..Default::default()
            })*/
            ..Default::default()
        };

        commands
            .spawn_bundle(PerspectiveCameraBundle {
                perspective_projection: perspective,
                transform: Transform::from_xyz(start_x, start_y, start_z).looking_at(
                    facing.camera_look_at(&Vec3::new(start_x, start_y, start_z)),
                    Vec3::new(0.0, 0.0, 1.0),
                ),
                camera,
                ..Default::default()
            })
            .insert(MapWander {})
            .insert(WanderCamera {});

        // Setup the player
        commands.spawn().insert(WanderingPlayer {
            x: tile_x as i32,
            y: tile_y as i32,
            facing,
        });
    }
}

pub fn exit_play_region(mut commands: Commands, cleanup: Query<(Entity, &MapWander)>) {
    cleanup
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn());
}

pub fn rebuild_region_map(
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