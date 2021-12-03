use crate::{module::MaterialDefinition, region::region_map::RegionBoundaryType};
use bevy_egui::egui::{Color32, Stroke};
use std::collections::HashMap;

pub struct RenderStrokes<'a> {
    pub full: Stroke,
    pub none: Stroke,
    pub highlight: Stroke,
    mats: &'a HashMap<usize, (String, MaterialDefinition)>,
}

impl<'a> RenderStrokes<'a> {
    pub fn default(mats: &'a HashMap<usize, (String, MaterialDefinition)>) -> Self {
        Self {
            full: Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 255)),
            none: Stroke::new(1.0, Color32::from_rgba_premultiplied(32, 32, 32, 255)),
            highlight: Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 0, 255)),
            mats,
        }
    }

    pub fn wall_type(&self, wall: (RegionBoundaryType, u32)) -> Stroke {
        match wall.0 {
            RegionBoundaryType::Wall | RegionBoundaryType::Opening => {
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

    pub fn fill(&self, mat_idx: usize) -> Color32 {
        match self.mats.get(&mat_idx) {
            Some((_, MaterialDefinition::Color { r, g, b })) => Color32::from_rgb(*r, *g, *b),
            Some((_, MaterialDefinition::Pbr { display_color, .. })) => {
                Color32::from_rgb(display_color.0, display_color.1, display_color.2)
            }
            _ => Color32::from_rgb(64, 64, 64),
        }
    }
}
