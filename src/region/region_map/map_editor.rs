use super::RegionMap;
use crate::{
    game_states::WanderResource,
    region::{
        region_map::{RegionBoundaryType, EAST, NORTH, SOUTH, WEST},
        Direction,
    },
};
use bevy_egui::egui::{
    emath::{self, RectTransform},
    Color32, CtxRef, Frame, Painter, PointerButton, Pos2, Rect, Response, Sense, Stroke, Ui,
    Window,
};

pub struct MapEditorSettings {
    fill_walls: bool,
}

impl MapEditorSettings {
    pub fn default() -> Self {
        Self { fill_walls: true }
    }
}

pub struct MapEditor<'a> {
    pub map: &'a mut RegionMap,
    pub settings: &'a mut MapEditorSettings,
}

impl<'a> MapEditor<'a> {
    pub fn render(ctx: &CtxRef, wander: &mut WanderResource) {
        Window::new("Map")
            .default_size(bevy_egui::egui::vec2(512.0, 512.0))
            .scroll(false)
            .show(ctx, |ui| {
                ui.checkbox(&mut wander.editor_settings.fill_walls, "Double-Sided Walls");
                ui.text_edit_singleline(&mut wander.map.name);
                Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    let mut ed = MapEditor {
                        map: &mut wander.map,
                        settings: &mut wander.editor_settings,
                    };
                    ed.ui_content(ui)
                });
            });
    }

    fn ui_content(&mut self, ui: &mut Ui) -> bevy_egui::egui::Response {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            Sense::union(Sense::click(), Sense::hover()),
        );

        let strokes = RenderStrokes::default();
        let scale = Scaling::new(&response, &self.map);

        self.draw_base_map(&painter, &scale, &strokes);

        if let Some(pointer_pos) = response.hover_pos() {
            self.interact(&scale, &strokes, pointer_pos, &painter, &response);
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
                    strokes.wall_type(tile.boundaries[NORTH].0),
                );
                painter.line_segment(
                    self.south_wall_line(x, y, scale),
                    strokes.wall_type(tile.boundaries[SOUTH].0),
                );
                painter.line_segment(
                    self.east_wall_line(x, y, scale),
                    strokes.wall_type(tile.boundaries[EAST].0),
                );
                painter.line_segment(
                    self.west_wall_line(x, y, scale),
                    strokes.wall_type(tile.boundaries[WEST].0),
                );
            }
        }
    }

    fn interact(
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
                    self.interact_click(wall.tile_x, wall.tile_y, WEST, response);
                }
                Direction::East => {
                    painter.line_segment(
                        self.east_wall_line(wall.tile_x, wall.tile_y, &scale),
                        strokes.highlight,
                    );
                    self.interact_click(wall.tile_x, wall.tile_y, EAST, response);
                }
                Direction::North => {
                    painter.line_segment(
                        self.north_wall_line(wall.tile_x, wall.tile_y, &scale),
                        strokes.highlight,
                    );
                    self.interact_click(wall.tile_x, wall.tile_y, NORTH, response);
                }
                Direction::South => {
                    painter.line_segment(
                        self.south_wall_line(wall.tile_x, wall.tile_y, &scale),
                        strokes.highlight,
                    );
                    self.interact_click(wall.tile_x, wall.tile_y, SOUTH, response);
                }
            }
        }
    }

    fn interact_click(&mut self, x: u32, y: u32, boundary: usize, response: &Response) {
        let tile_idx = ((self.map.size.0 * y) + x) as usize;
        if response.clicked_by(PointerButton::Primary) {
            self.map.tiles[tile_idx].boundaries[boundary].0 = RegionBoundaryType::WALL;
            if self.settings.fill_walls {
                self.reciprocal_click(x, y, boundary, RegionBoundaryType::WALL);
            }
            self.map.needs_rebuild = true;
        } else if response.clicked_by(PointerButton::Secondary) {
            self.map.tiles[tile_idx].boundaries[boundary].0 = RegionBoundaryType::NONE;
            if self.settings.fill_walls {
                self.reciprocal_click(x, y, boundary, RegionBoundaryType::NONE);
            }
            self.map.needs_rebuild = true;
        }
    }

    fn reciprocal_click(&mut self, x: u32, y: u32, boundary: usize, new_wall: RegionBoundaryType) {
        if boundary == NORTH && y > 0 {
            let tile_idx = ((self.map.size.0 * (y - 1)) + x) as usize;
            self.map.tiles[tile_idx].boundaries[SOUTH].0 = new_wall;
        } else if boundary == SOUTH && y < self.map.size.1 - 1 {
            let tile_idx = ((self.map.size.0 * (y + 1)) + x) as usize;
            self.map.tiles[tile_idx].boundaries[NORTH].0 = new_wall;
        } else if boundary == WEST && x > 0 {
            let tile_idx = ((self.map.size.0 * y) + x - 1) as usize;
            self.map.tiles[tile_idx].boundaries[EAST].0 = new_wall;
        } else if boundary == EAST && x < self.map.size.0 - 1 {
            let tile_idx = ((self.map.size.0 * y) + x + 1) as usize;
            self.map.tiles[tile_idx].boundaries[WEST].0 = new_wall;
        }
    }
}

struct RenderStrokes {
    full: Stroke,
    none: Stroke,
    highlight: Stroke,
}

impl RenderStrokes {
    fn default() -> Self {
        Self {
            full: Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 255)),
            none: Stroke::new(1.0, Color32::from_rgba_premultiplied(128, 128, 128, 128)),
            highlight: Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 0, 255)),
        }
    }

    fn wall_type(&self, wall: RegionBoundaryType) -> Stroke {
        match wall {
            RegionBoundaryType::WALL => self.full,
            _ => self.none,
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
