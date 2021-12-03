mod editor_mode;
pub use editor_mode::MapEditorMode;
mod editor_settings;
pub use editor_settings::MapEditorSettings;
mod render_strokes;
use render_strokes::RenderStrokes;
mod scaling;
use scaling::Scaling;
mod wall_interaction;
use wall_interaction::MapWallInteraction;
mod wall_lines;
use super::{RegionMap, RegionTileType};
use crate::{
    module::{Direction, MaterialDefinition, Module},
    region::region_map::RegionBoundaryType,
};
use bevy_egui::egui::{
    Align2, Color32, CtxRef, Frame, Painter, PointerButton, Pos2, Response, Sense, Stroke, Ui,
    Window,
};
use std::collections::HashMap;
use wall_lines::{wall_line, wall_opening};

pub struct MapEditor<'a> {
    pub map: &'a mut RegionMap,
    pub settings: &'a mut MapEditorSettings,
}

impl<'a> MapEditor<'a> {
    pub fn render_in_module(
        ctx: &CtxRef,
        editor_settings: &mut MapEditorSettings,
        module: &mut Module,
        map_id: usize,
    ) {
        let map = module.maps.get_mut(&map_id).unwrap();
        let mats = module.materials.clone();
        Window::new(format!("Map: {}", map.name))
            .default_size(bevy_egui::egui::vec2(512.0, 512.0))
            .scroll(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    ui.radio_value(&mut editor_settings.mode, MapEditorMode::Walls, "Wall");
                    ui.radio_value(&mut editor_settings.mode, MapEditorMode::Floor, "Floor");
                    ui.radio_value(&mut editor_settings.mode, MapEditorMode::Ceiling, "Ceiling");
                    ui.radio_value(&mut editor_settings.mode, MapEditorMode::Start, "Start");
                    ui.radio_value(&mut editor_settings.mode, MapEditorMode::Opening, "Opening");
                });
                ui.checkbox(&mut editor_settings.fill_walls, "Double-Sided Walls");

                let current_label = mats[&editor_settings.material].0.clone();
                bevy_egui::egui::ComboBox::from_label("Material")
                    .selected_text(current_label)
                    .show_ui(ui, |ui| {
                        for (i, v) in mats.iter() {
                            ui.selectable_value(&mut editor_settings.material, *i, v.0.clone());
                        }
                    });

                ui.separator();
                ui.text_edit_singleline(&mut map.name);
                Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    let mut ed = MapEditor {
                        map,
                        settings: editor_settings,
                    };
                    ed.ui_content(ui, &mats)
                });
            });
    }

    fn ui_content(
        &mut self,
        ui: &mut Ui,
        mats: &HashMap<usize, (String, MaterialDefinition)>,
    ) -> bevy_egui::egui::Response {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            Sense::union(Sense::click(), Sense::hover()),
        );

        let strokes = RenderStrokes::default(mats);
        let scale = Scaling::new(&response, &self.map);

        self.draw_base_map(&painter, &scale, &strokes);

        if let Some(pointer_pos) = response.hover_pos() {
            match self.settings.mode {
                MapEditorMode::Walls | MapEditorMode::Opening => {
                    self.wall_interact(&scale, &strokes, pointer_pos, &painter, &response)
                }
                MapEditorMode::Floor => self.floor_interact(&scale, pointer_pos, &response),
                MapEditorMode::Ceiling => self.ceiling_interact(&scale, pointer_pos, &response),
                MapEditorMode::Start => self.start_interact(&scale, pointer_pos, &response),
            }
        }

        response
    }

    fn draw_base_map(&self, painter: &Painter, scale: &Scaling, strokes: &RenderStrokes) {
        for y in 0..self.map.size.1 {
            for x in 0..self.map.size.0 {
                let tile_idx = ((self.map.size.0 * y) + x) as usize;
                let tile = &self.map.tiles[tile_idx];

                for dir in 0..4 {
                    let direction: Direction = dir.into();
                    match tile.boundaries[dir].0 {
                        RegionBoundaryType::Opening => {
                            wall_opening(direction, x, y, scale)
                                .iter()
                                .for_each(|segment| {
                                    painter.line_segment(
                                        *segment,
                                        strokes.wall_type(
                                            tile.boundaries[Direction::North.to_exit_index()],
                                        ),
                                    );
                                });
                        }
                        RegionBoundaryType::None | RegionBoundaryType::Wall => {
                            painter.line_segment(
                                wall_line(direction, x, y, scale),
                                strokes.wall_type(tile.boundaries[dir]),
                            );
                        }
                    }
                }

                // Display Floors
                if self.settings.mode == MapEditorMode::Floor
                    && tile.tile_type == RegionTileType::Floor
                {
                    let px = x as f32 * scale.box_x;
                    let py = y as f32 * scale.box_y;
                    let x20 = scale.x10 * 8.0;
                    let y20 = scale.y10 * 8.0;
                    painter.rect(
                        bevy_egui::egui::Rect::from_two_pos(
                            scale.to_screen
                                * Pos2 {
                                    x: px + x20,
                                    y: py + y20,
                                },
                            scale.to_screen
                                * Pos2 {
                                    x: (px + scale.box_x) - x20,
                                    y: (py + scale.box_y) - y20,
                                },
                        ),
                        1.0,
                        strokes.fill(tile.floor_material as usize),
                        Stroke::new(4.0, Color32::BLACK),
                    );
                }

                // Display Ceilings
                if self.settings.mode == MapEditorMode::Ceiling && tile.has_ceiling {
                    let px = x as f32 * scale.box_x;
                    let py = y as f32 * scale.box_y;
                    let x20 = scale.x10 * 8.0;
                    let y20 = scale.y10 * 8.0;
                    painter.rect(
                        bevy_egui::egui::Rect::from_two_pos(
                            scale.to_screen
                                * Pos2 {
                                    x: px + x20,
                                    y: py + y20,
                                },
                            scale.to_screen
                                * Pos2 {
                                    x: (px + scale.box_x) - x20,
                                    y: (py + scale.box_y) - y20,
                                },
                        ),
                        1.0,
                        strokes.fill(tile.ceiling_material as usize),
                        Stroke::new(4.0, Color32::BLACK),
                    );
                }

                // Start loc
                if self.settings.mode == MapEditorMode::Start {
                    let px =
                        (self.map.starting_location.0 as f32 * scale.box_x) + (scale.box_x / 2.0);
                    let py =
                        (self.map.starting_location.1 as f32 * scale.box_y) + (scale.box_y / 2.0);
                    painter.text(
                        scale.to_screen * Pos2 { x: px, y: py },
                        Align2::CENTER_CENTER,
                        "S",
                        bevy_egui::egui::TextStyle::Monospace,
                        Color32::YELLOW,
                    );
                }

                // Current player position
                if let Some((x, y)) = self.settings.highlight_player {
                    let px = (x as f32 * scale.box_x) + (scale.box_x / 2.0);
                    let py = (y as f32 * scale.box_y) + (scale.box_y / 2.0);
                    painter.text(
                        scale.to_screen * Pos2 { x: px, y: py },
                        Align2::CENTER_CENTER,
                        "@",
                        bevy_egui::egui::TextStyle::Monospace,
                        Color32::YELLOW,
                    );
                }
            }
        }
    }

    fn start_interact(&mut self, scale: &Scaling, pointer_pos: Pos2, response: &Response) {
        if response.clicked_by(PointerButton::Primary) {
            let pos = MapWallInteraction::new(scale, pointer_pos, &self.map);
            self.map.starting_location.0 = pos.tile_x;
            self.map.starting_location.1 = pos.tile_y;
        }
    }

    fn floor_interact(&mut self, scale: &Scaling, pointer_pos: Pos2, response: &Response) {
        if response.clicked_by(PointerButton::Primary) {
            let pos = MapWallInteraction::new(scale, pointer_pos, &self.map);
            let tile_idx = ((self.map.size.0 * pos.tile_y) + pos.tile_x) as usize;
            self.map.tiles[tile_idx].floor_material = self.settings.material as u32;
            self.map.tiles[tile_idx].tile_type = RegionTileType::Floor;
            self.map.needs_rebuild = true;
        }
        if response.clicked_by(PointerButton::Secondary) {
            let pos = MapWallInteraction::new(scale, pointer_pos, &self.map);
            let tile_idx = ((self.map.size.0 * pos.tile_y) + pos.tile_x) as usize;
            self.map.tiles[tile_idx].tile_type = RegionTileType::Empty;
            self.map.needs_rebuild = true;
        }
    }

    fn ceiling_interact(&mut self, scale: &Scaling, pointer_pos: Pos2, response: &Response) {
        if response.clicked_by(PointerButton::Primary) {
            let pos = MapWallInteraction::new(scale, pointer_pos, &self.map);
            let tile_idx = ((self.map.size.0 * pos.tile_y) + pos.tile_x) as usize;
            self.map.tiles[tile_idx].ceiling_material = self.settings.material as u32;
            self.map.tiles[tile_idx].has_ceiling = true;
            self.map.needs_rebuild = true;
        }
        if response.clicked_by(PointerButton::Secondary) {
            let pos = MapWallInteraction::new(scale, pointer_pos, &self.map);
            let tile_idx = ((self.map.size.0 * pos.tile_y) + pos.tile_x) as usize;
            self.map.tiles[tile_idx].has_ceiling = false;
            self.map.needs_rebuild = true;
        }
    }

    fn wall_interact(
        &mut self,
        scale: &Scaling,
        strokes: &RenderStrokes,
        pointer_pos: Pos2,
        painter: &Painter,
        response: &Response,
    ) {
        let wall = MapWallInteraction::new(scale, pointer_pos, &self.map);
        if let Some(direction) = wall.selected_wall {
            painter.line_segment(
                wall_line(direction, wall.tile_x, wall.tile_y, &scale),
                strokes.highlight,
            );
            self.wall_interact_click(
                wall.tile_x,
                wall.tile_y,
                direction.to_exit_index(),
                response,
            );
        }
    }

    fn wall_interact_click(&mut self, x: u32, y: u32, boundary: usize, response: &Response) {
        let tile_idx = ((self.map.size.0 * y) + x) as usize;
        if response.clicked_by(PointerButton::Primary) {
            self.map.tiles[tile_idx].boundaries[boundary].0 = match self.settings.mode {
                MapEditorMode::Opening => RegionBoundaryType::Opening,
                _ => RegionBoundaryType::Wall,
            };
            self.map.tiles[tile_idx].boundaries[boundary].1 = self.settings.material as u32;
            if self.settings.fill_walls {
                self.wall_reciprocal_click(
                    x,
                    y,
                    boundary,
                    match self.settings.mode {
                        MapEditorMode::Opening => RegionBoundaryType::Opening,
                        _ => RegionBoundaryType::Wall,
                    },
                );
            }
            self.map.needs_rebuild = true;
        } else if response.clicked_by(PointerButton::Secondary) {
            self.map.tiles[tile_idx].boundaries[boundary].0 = RegionBoundaryType::None;
            if self.settings.fill_walls {
                self.wall_reciprocal_click(x, y, boundary, RegionBoundaryType::None);
            }
            self.map.needs_rebuild = true;
        }
    }

    fn wall_reciprocal_click(
        &mut self,
        x: u32,
        y: u32,
        boundary: usize,
        new_wall: RegionBoundaryType,
    ) {
        if boundary == Direction::North.to_exit_index() && y > 0 {
            let tile_idx = ((self.map.size.0 * (y - 1)) + x) as usize;
            self.map.tiles[tile_idx].boundaries[Direction::South.to_exit_index()].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[Direction::South.to_exit_index()].1 =
                self.settings.material as u32;
        } else if boundary == Direction::South.to_exit_index() && y < self.map.size.1 - 1 {
            let tile_idx = ((self.map.size.0 * (y + 1)) + x) as usize;
            self.map.tiles[tile_idx].boundaries[Direction::North.to_exit_index()].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[Direction::North.to_exit_index()].1 =
                self.settings.material as u32;
        } else if boundary == Direction::West.to_exit_index() && x > 0 {
            let tile_idx = ((self.map.size.0 * y) + x - 1) as usize;
            self.map.tiles[tile_idx].boundaries[Direction::East.to_exit_index()].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[Direction::East.to_exit_index()].1 =
                self.settings.material as u32;
        } else if boundary == Direction::East.to_exit_index() && x < self.map.size.0 - 1 {
            let tile_idx = ((self.map.size.0 * y) + x + 1) as usize;
            self.map.tiles[tile_idx].boundaries[Direction::West.to_exit_index()].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[Direction::West.to_exit_index()].1 =
                self.settings.material as u32;
        }
    }
}
