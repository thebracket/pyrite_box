use crate::region::region_map::RegionMap;
use bevy_egui::egui::{emath::RectTransform, Pos2, Rect, Response};

pub struct Scaling {
    pub box_x: f32,
    pub box_y: f32,
    pub x10: f32,
    pub y10: f32,
    pub to_screen: RectTransform,
    pub from_screen: RectTransform,
}

impl Scaling {
    pub fn new(response: &Response, map: &RegionMap) -> Self {
        let box_x: f32 = 1.0 / map.size.0 as f32;
        let box_y: f32 = 1.0 / map.size.1 as f32;
        let to_screen = RectTransform::from_to(
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
