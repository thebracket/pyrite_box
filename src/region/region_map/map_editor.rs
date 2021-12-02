use std::collections::HashMap;

use super::{RegionMap, RegionTileType};
use crate::{
    module::{MaterialDefinition, Module},
    region::{
        region_map::{RegionBoundaryType, EAST, NORTH, SOUTH, WEST},
        Direction,
    },
};
use bevy_egui::egui::{
    emath::{self, RectTransform},
    Align2, Color32, CtxRef, Frame, Painter, PointerButton, Pos2, Rect, Response, Sense, Stroke,
    Ui, Window,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapEditorMode {
    Walls,
    Floor,
    Ceiling,
    Start,
}

#[derive(Clone)]
pub struct MapEditorSettings {
    mode: MapEditorMode,
    fill_walls: bool,
    material: usize,
}

impl MapEditorSettings {
    pub fn default() -> Self {
        Self {
            mode: MapEditorMode::Walls,
            fill_walls: true,
            material: 0,
        }
    }
}

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
                MapEditorMode::Walls => {
                    self.wall_interact(&scale, &strokes, pointer_pos, &painter, &response)
                }
                MapEditorMode::Floor => self.floor_interact(&scale, pointer_pos, &response),
                MapEditorMode::Ceiling => self.ceiling_interact(&scale, pointer_pos, &response),
                MapEditorMode::Start => self.start_interact(&scale, pointer_pos, &response),
            }
        }

        response
    }

    fn north_wall_line(&self, x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
        let px = x as f32 * scale.box_x;
        let py = y as f32 * scale.box_y;
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.y10,
                },
        ]
    }

    fn south_wall_line(&self, x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
        let px = x as f32 * scale.box_x;
        let py = y as f32 * scale.box_y;
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
        ]
    }

    fn west_wall_line(&self, x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
        let px = x as f32 * scale.box_x;
        let py = y as f32 * scale.box_y;
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
        ]
    }

    fn east_wall_line(&self, x: u32, y: u32, scale: &Scaling) -> [Pos2; 2] {
        let px = x as f32 * scale.box_x;
        let py = y as f32 * scale.box_y;
        [
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.y10,
                },
            scale.to_screen
                * Pos2 {
                    x: px + scale.box_x - scale.x10,
                    y: py + scale.box_y - scale.y10,
                },
        ]
    }

    fn draw_base_map(&self, painter: &Painter, scale: &Scaling, strokes: &RenderStrokes) {
        for y in 0..self.map.size.1 {
            for x in 0..self.map.size.0 {
                let tile_idx = ((self.map.size.0 * y) + x) as usize;
                let tile = &self.map.tiles[tile_idx];

                painter.line_segment(
                    self.north_wall_line(x, y, scale),
                    strokes.wall_type(tile.boundaries[NORTH]),
                );
                painter.line_segment(
                    self.south_wall_line(x, y, scale),
                    strokes.wall_type(tile.boundaries[SOUTH]),
                );
                painter.line_segment(
                    self.east_wall_line(x, y, scale),
                    strokes.wall_type(tile.boundaries[EAST]),
                );
                painter.line_segment(
                    self.west_wall_line(x, y, scale),
                    strokes.wall_type(tile.boundaries[WEST]),
                );

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
            match direction {
                Direction::West => {
                    painter.line_segment(
                        self.west_wall_line(wall.tile_x, wall.tile_y, &scale),
                        strokes.highlight,
                    );
                    self.wall_interact_click(wall.tile_x, wall.tile_y, WEST, response);
                }
                Direction::East => {
                    painter.line_segment(
                        self.east_wall_line(wall.tile_x, wall.tile_y, &scale),
                        strokes.highlight,
                    );
                    self.wall_interact_click(wall.tile_x, wall.tile_y, EAST, response);
                }
                Direction::North => {
                    painter.line_segment(
                        self.north_wall_line(wall.tile_x, wall.tile_y, &scale),
                        strokes.highlight,
                    );
                    self.wall_interact_click(wall.tile_x, wall.tile_y, NORTH, response);
                }
                Direction::South => {
                    painter.line_segment(
                        self.south_wall_line(wall.tile_x, wall.tile_y, &scale),
                        strokes.highlight,
                    );
                    self.wall_interact_click(wall.tile_x, wall.tile_y, SOUTH, response);
                }
            }
        }
    }

    fn wall_interact_click(&mut self, x: u32, y: u32, boundary: usize, response: &Response) {
        let tile_idx = ((self.map.size.0 * y) + x) as usize;
        if response.clicked_by(PointerButton::Primary) {
            self.map.tiles[tile_idx].boundaries[boundary].0 = RegionBoundaryType::Wall;
            self.map.tiles[tile_idx].boundaries[boundary].1 = self.settings.material as u32;
            if self.settings.fill_walls {
                self.wall_reciprocal_click(x, y, boundary, RegionBoundaryType::Wall);
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
        if boundary == NORTH && y > 0 {
            let tile_idx = ((self.map.size.0 * (y - 1)) + x) as usize;
            self.map.tiles[tile_idx].boundaries[SOUTH].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[SOUTH].1 = self.settings.material as u32;
        } else if boundary == SOUTH && y < self.map.size.1 - 1 {
            let tile_idx = ((self.map.size.0 * (y + 1)) + x) as usize;
            self.map.tiles[tile_idx].boundaries[NORTH].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[NORTH].1 = self.settings.material as u32;
        } else if boundary == WEST && x > 0 {
            let tile_idx = ((self.map.size.0 * y) + x - 1) as usize;
            self.map.tiles[tile_idx].boundaries[EAST].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[EAST].1 = self.settings.material as u32;
        } else if boundary == EAST && x < self.map.size.0 - 1 {
            let tile_idx = ((self.map.size.0 * y) + x + 1) as usize;
            self.map.tiles[tile_idx].boundaries[WEST].0 = new_wall;
            self.map.tiles[tile_idx].boundaries[WEST].1 = self.settings.material as u32;
        }
    }
}

struct RenderStrokes<'a> {
    full: Stroke,
    none: Stroke,
    highlight: Stroke,
    mats: &'a HashMap<usize, (String, MaterialDefinition)>,
}

impl<'a> RenderStrokes<'a> {
    fn default(mats: &'a HashMap<usize, (String, MaterialDefinition)>) -> Self {
        Self {
            full: Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 255)),
            none: Stroke::new(1.0, Color32::from_rgba_premultiplied(32, 32, 32, 255)),
            highlight: Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 0, 255)),
            mats,
        }
    }

    fn wall_type(&self, wall: (RegionBoundaryType, u32)) -> Stroke {
        match wall.0 {
            RegionBoundaryType::Wall => {
                let mat_idx = wall.1 as usize;
                if let Some((_, MaterialDefinition::Color { r, g, b })) = self.mats.get(&mat_idx) {
                    Stroke::new(1.0, Color32::from_rgb(*r, *g, *b))
                } else if let Some((_, MaterialDefinition::Pbr { display_color, .. })) =
                    self.mats.get(&mat_idx)
                {
                    Stroke::new(
                        1.0,
                        Color32::from_rgb(display_color.0, display_color.1, display_color.2),
                    )
                } else {
                    self.full
                }
            }
            _ => self.none,
        }
    }

    fn fill(&self, mat_idx: usize) -> Color32 {
        match self.mats.get(&mat_idx) {
            Some((_, MaterialDefinition::Color { r, g, b })) => Color32::from_rgb(*r, *g, *b),
            Some((_, MaterialDefinition::Pbr { display_color, .. })) => {
                Color32::from_rgb(display_color.0, display_color.1, display_color.2)
            }
            _ => Color32::from_rgb(64, 64, 64),
        }
    }
}

struct Scaling {
    box_x: f32,
    box_y: f32,
    x10: f32,
    y10: f32,
    to_screen: RectTransform,
    from_screen: RectTransform,
}

impl Scaling {
    fn new(response: &Response, map: &RegionMap) -> Self {
        let box_x: f32 = 1.0 / map.size.0 as f32;
        let box_y: f32 = 1.0 / map.size.1 as f32;
        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        Self {
            box_x,
            box_y,
            x10: box_x / 30.0,
            y10: box_y / 30.0,
            to_screen,
            from_screen: to_screen.inverse(),
        }
    }
}

struct MapWallInteraction {
    tile_x: u32,
    tile_y: u32,
    selected_wall: Option<Direction>,
}

impl MapWallInteraction {
    fn new(scale: &Scaling, pointer_pos: Pos2, map: &RegionMap) -> Self {
        let canvas_pos = scale.from_screen * pointer_pos;
        let tile_x = f32::min(
            (canvas_pos.x * map.size.0 as f32).floor(),
            map.size.0 as f32 - 1.0,
        );
        let tile_y = f32::min(
            (canvas_pos.y * map.size.0 as f32).floor(),
            map.size.1 as f32 - 1.0,
        );

        let wall = if canvas_pos.x < (tile_x * scale.box_x) + (10.0 * scale.x10) {
            Some(Direction::West)
        } else if canvas_pos.x > ((tile_x + 1.0) * scale.box_x) - (10.0 * scale.x10) {
            Some(Direction::East)
        } else if canvas_pos.y < (tile_y * scale.box_y) + (10.0 * scale.y10) {
            Some(Direction::North)
        } else if canvas_pos.y > ((tile_y + 1.0) * scale.box_y) - (10.0 * scale.y10) {
            Some(Direction::South)
        } else {
            None
        };

        Self {
            tile_x: tile_x as u32,
            tile_y: tile_y as u32,
            selected_wall: wall,
        }
    }
}
