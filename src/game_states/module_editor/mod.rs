use super::ModuleSelector;
use crate::{
    module::{game_events::EventPicker, Module},
    region::region_map::{
        map_editor::{MapEditor, MapEditorSettings},
        RegionMap,
    },
};
use bevy::prelude::*;
use bevy_egui::EguiContext;
mod events;
mod maps;
mod materials;
mod menu;
mod module_info;

pub struct ModuleResource {
    pub module: Module,
    show_info: bool,
    show_materials: bool,
    current_material: usize,
    new_material_name: String,
    show_maps: bool,
    new_map: RegionMap,
    editing_map: Option<usize>,
    editor_settings: MapEditorSettings,
    show_events: bool,
    new_event_tag: String,
    editing_event: Option<String>,
    new_event_step: EventPicker,
}

pub fn module_editor(egui_context: ResMut<EguiContext>, mut module_res: ResMut<ModuleResource>) {
    menu::editor_menu(&egui_context, &mut module_res);
    module_info::module_info(&egui_context, &mut module_res);
    materials::material_editor(&egui_context, &mut module_res);
    maps::maps(&egui_context, &mut module_res);

    if let Some(map_id) = module_res.editing_map {
        let mut es = module_res.editor_settings.clone();
        MapEditor::render_in_module(egui_context.ctx(), &mut es, &mut module_res.module, map_id);
        module_res.editor_settings = es;
    }

    events::events(&egui_context, &mut module_res);
    events::event_editor(&egui_context, &mut module_res);
}

pub fn resume_module_editor(mut commands: Commands, startup: Res<ModuleSelector>) {
    if let Some(module) = &startup.module {
        commands.insert_resource(ModuleResource {
            //module: Module::load(&filename),
            module: module.clone(),
            show_info: false,
            show_materials: false,
            current_material: 0,
            new_material_name: "New Material".to_string(),
            show_maps: false,
            new_map: RegionMap::default(),
            editing_map: None,
            editor_settings: MapEditorSettings::default(),
            show_events: false,
            new_event_tag: String::new(),
            editing_event: None,
            new_event_step: EventPicker::LogText,
        });
    } else {
        commands.insert_resource(ModuleResource {
            module: Module::default(),
            show_info: false,
            show_materials: false,
            current_material: 0,
            new_material_name: "New Material".to_string(),
            show_maps: false,
            new_map: RegionMap::default(),
            editing_map: None,
            editor_settings: MapEditorSettings::default(),
            show_events: false,
            new_event_tag: String::new(),
            editing_event: None,
            new_event_step: EventPicker::LogText,
        });
    }
}

pub fn exit_module_editor(mut commands: Commands) {
    commands.remove_resource::<ModuleResource>();
}
