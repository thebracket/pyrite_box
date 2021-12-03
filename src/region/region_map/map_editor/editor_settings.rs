use super::editor_mode::MapEditorMode;

#[derive(Clone)]
pub struct MapEditorSettings {
    pub mode: MapEditorMode,
    pub fill_walls: bool,
    pub material: usize,
    pub highlight_player: Option<(i32,i32)>,
}

impl MapEditorSettings {
    pub fn default() -> Self {
        Self {
            mode: MapEditorMode::Walls,
            fill_walls: true,
            material: 0,
            highlight_player: None,
        }
    }
}
