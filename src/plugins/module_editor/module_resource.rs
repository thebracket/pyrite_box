use crate::{
    modules::{game_events::EventPicker, Module},
    region::region_map::RegionMap,
    state::MapEditorSettings,
};

pub struct ModuleResource {
    pub module: Module,
    pub show_info: bool,
    pub show_materials: bool,
    pub current_material: usize,
    pub new_material_name: String,
    pub show_maps: bool,
    pub new_map: RegionMap,
    pub editing_map: Option<usize>,
    pub editor_settings: MapEditorSettings,
    pub show_events: bool,
    pub new_event_tag: String,
    pub editing_event: Option<String>,
    pub new_event_step: EventPicker,
}
