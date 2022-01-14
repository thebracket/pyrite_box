use super::{MapEditorSettings, WanderInput};
use crate::modules::Module;

pub struct WanderResource {
    pub module: Module,
    pub map_idx: usize,
    pub editor_settings: MapEditorSettings,
    pub show_editor: bool,
    pub allow_movement: bool,
    pub script_input: Option<WanderInput>,
}
