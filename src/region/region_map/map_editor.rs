use bevy_egui::egui::{emath, Color32, PointerButton, Pos2, Rect, Sense, Stroke, Ui};

use crate::region::region_map::{RegionBoundaryType, EAST, NORTH, SOUTH, WEST};

use super::RegionMap;

pub struct MapEditor<'a> {
    pub map: &'a mut RegionMap,
}

impl<'a> MapEditor<'a> {
    pub fn ui_content(&mut self, ui: &mut Ui) -> bevy_egui::egui::Response {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            Sense::union(Sense::click(), Sense::hover()),
        );

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        let full = Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 255));
        let none = Stroke::new(1.0, Color32::from_rgba_premultiplied(128, 128, 128, 128));

        let box_x: f32 = 1.0 / self.map.size.0 as f32;
        let box_y: f32 = 1.0 / self.map.size.1 as f32;
        let x10 = box_x / 30.0;
        let y10 = box_y / 30.0;

        for y in 0..self.map.size.1 {
            for x in 0..self.map.size.0 {
                let px = x as f32 * box_x;
                let py = y as f32 * box_y;

                let tile_idx = ((self.map.size.0 * y) + x) as usize;

                if self.map.tiles[tile_idx].boundaries[NORTH].0 == RegionBoundaryType::WALL {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + y10,
                                },
                        ],
                        full,
                    );
                } else {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + y10,
                                },
                        ],
                        none,
                    );
                }

                if self.map.tiles[tile_idx].boundaries[SOUTH].0 == RegionBoundaryType::WALL {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + box_y - y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + box_y - y10,
                                },
                        ],
                        full,
                    );
                } else {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + box_y - y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + box_y - y10,
                                },
                        ],
                        none,
                    );
                }

                if self.map.tiles[tile_idx].boundaries[WEST].0 == RegionBoundaryType::WALL {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + box_y - y10,
                                },
                        ],
                        full,
                    );
                } else {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + x10,
                                    y: py + box_y - y10,
                                },
                        ],
                        none,
                    );
                }

                if self.map.tiles[tile_idx].boundaries[EAST].0 == RegionBoundaryType::WALL {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + box_y - y10,
                                },
                        ],
                        full,
                    );
                } else {
                    painter.line_segment(
                        [
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + y10,
                                },
                            to_screen
                                * Pos2 {
                                    x: px + box_x - x10,
                                    y: py + box_y - y10,
                                },
                        ],
                        none,
                    );
                }
            }
        }
        //response.mark_changed();

        if let Some(pointer_pos) = response.hover_pos() {
            let canvas_pos = from_screen * pointer_pos;
            let tile_x = f32::min(
                (canvas_pos.x * self.map.size.0 as f32).floor(),
                self.map.size.0 as f32 - 1.0,
            );
            let tile_y = f32::min(
                (canvas_pos.y * self.map.size.0 as f32).floor(),
                self.map.size.1 as f32 - 1.0,
            );

            let highlight = Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 0, 255));
            if canvas_pos.x < (tile_x * box_x) + (10.0 * x10) {
                // Left side
                painter.line_segment(
                    [
                        to_screen
                            * Pos2 {
                                x: (tile_x * box_x) + x10,
                                y: tile_y * box_y,
                            },
                        to_screen
                            * Pos2 {
                                x: (tile_x * box_x) + x10,
                                y: (tile_y + 1.0) * box_y,
                            },
                    ],
                    highlight,
                );
                if response.clicked_by(PointerButton::Primary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[WEST].0 = RegionBoundaryType::WALL;
                }
                if response.clicked_by(PointerButton::Secondary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[WEST].0 = RegionBoundaryType::NONE;
                }
            } else if canvas_pos.x > ((tile_x + 1.0) * box_x) - (10.0 * x10) {
                // Right side
                painter.line_segment(
                    [
                        to_screen
                            * Pos2 {
                                x: ((tile_x + 1.0) * box_x) - x10,
                                y: tile_y * box_y,
                            },
                        to_screen
                            * Pos2 {
                                x: ((tile_x + 1.0) * box_x) - x10,
                                y: (tile_y + 1.0) * box_y,
                            },
                    ],
                    highlight,
                );
                if response.clicked_by(PointerButton::Primary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[EAST].0 = RegionBoundaryType::WALL;
                }
                if response.clicked_by(PointerButton::Secondary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[EAST].0 = RegionBoundaryType::NONE;
                }
            } else if canvas_pos.y < (tile_y * box_y) + (10.0 * y10) {
                // Top side
                painter.line_segment(
                    [
                        to_screen
                            * Pos2 {
                                x: (tile_x * box_x) + x10,
                                y: (tile_y * box_y) + y10,
                            },
                        to_screen
                            * Pos2 {
                                x: ((tile_x + 1.0) * box_x) - x10,
                                y: (tile_y * box_y) + y10,
                            },
                    ],
                    highlight,
                );
                if response.clicked_by(PointerButton::Primary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[NORTH].0 = RegionBoundaryType::WALL;
                }
                if response.clicked_by(PointerButton::Secondary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[NORTH].0 = RegionBoundaryType::NONE;
                }
            } else if canvas_pos.y > ((tile_y + 1.0) * box_y) - (10.0 * y10) {
                // Bottom side
                painter.line_segment(
                    [
                        to_screen
                            * Pos2 {
                                x: (tile_x * box_x) + x10,
                                y: ((tile_y + 1.0) * box_y) - y10,
                            },
                        to_screen
                            * Pos2 {
                                x: ((tile_x + 1.0) * box_x) + x10,
                                y: ((tile_y + 1.0) * box_y) - y10,
                            },
                    ],
                    highlight,
                );
                if response.clicked_by(PointerButton::Primary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[SOUTH].0 = RegionBoundaryType::WALL;
                }
                if response.clicked_by(PointerButton::Secondary) {
                    let tile_idx = ((tile_y as u32 * self.map.size.0) + tile_x as u32) as usize;
                    self.map.tiles[tile_idx].boundaries[SOUTH].0 = RegionBoundaryType::NONE;
                }
            }

            /*painter.line_segment(
                [ to_screen * Pos2{ x: tile_x * box_x, y: tile_y * box_y}, to_screen * Pos2{ x: (tile_x+1.0)*box_x, y: (tile_y+1.0)*box_y } ],
                highlight
            )*/
        }

        response
    }
}
