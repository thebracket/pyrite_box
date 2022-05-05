use super::{
    events::{event_editor, events},
    maps::maps,
    materials::material_editor,
    menu::editor_menu,
    module_info::module_info,
    module_resource::ModuleResource,
};
use crate::{
    modules::{game_events::EventPicker, Module},
    region::region_map::{map_editor::MapEditor, RegionMap},
    state::{MapEditorSettings, ModuleSelector},
};
use bevy::prelude::*;
use bevy_egui::EguiContext;

pub fn module_editor(mut egui_context: ResMut<EguiContext>, mut module_res: ResMut<ModuleResource>) {
    editor_menu(&mut egui_context, &mut module_res);
    module_info(&mut egui_context, &mut module_res);
    material_editor(&mut egui_context, &mut module_res);
    maps(&mut egui_context, &mut module_res);

    if let Some(map_id) = module_res.editing_map {
        let mut es = module_res.editor_settings.clone();
        MapEditor::render_in_module(egui_context.ctx_mut(), &mut es, &mut module_res.module, map_id);
        module_res.editor_settings = es;
    }

    events(&mut egui_context, &mut module_res);
    event_editor(&mut egui_context, &mut module_res);
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
