use super::ModuleSelector;
use crate::{
    module::{default_pbr, MaterialDefinition, Module, game_events::{GameEvent, EventPicker, GameEventStep}},
    region::region_map::{
        map_editor::{MapEditor, MapEditorSettings},
        RegionMap,
    },
};
use bevy::prelude::*;
use bevy_egui::egui::Widget;
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};

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
    egui::TopBottomPanel::top("menu_bar").show(egui_context.ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "Module Editor", |ui| {
                if ui.button("Module Info").clicked() {
                    module_res.show_info = !module_res.show_info;
                }
                if ui.button("Materials").clicked() {
                    module_res.show_materials = !module_res.show_materials;
                }
                if ui.button("Map Manager").clicked() {
                    module_res.show_maps = !module_res.show_maps;
                }
                if ui.button("Event Scripting").clicked() {
                    module_res.show_events = !module_res.show_events;
                }
                if ui.button("Save").clicked() {
                    module_res.module.save();
                }
            });
        });
    });

    if module_res.show_info {
        egui::Window::new("Module Editor")
            .auto_sized()
            .title_bar(true)
            .show(egui_context.ctx(), |ui| {
                ui.label("Module Name");
                ui.text_edit_singleline(&mut module_res.module.name);
                ui.label("Module Filename");
                ui.text_edit_singleline(&mut module_res.module.filename);
                ui.label("Module Description");
                ui.text_edit_multiline(&mut module_res.module.description);
                ui.label("Event Tag - On Start");
                ui.text_edit_singleline(&mut module_res.module.module_start_event);
            });
    }

    if module_res.show_materials {
        egui::Window::new("Material Editor")
            .title_bar(true)
            .show(egui_context.ctx(), |ui| {
                ui.label("ADD NEW MATERIAL");
                ui.text_edit_singleline(&mut module_res.new_material_name);
                if ui.button("Add Material").clicked() {
                    let id = module_res.module.next_material_index;
                    module_res.module.next_material_index += 1;
                    let name = module_res.new_material_name.clone();
                    module_res
                        .module
                        .materials
                        .insert(id, (name, MaterialDefinition::Color { r: 0, g: 0, b: 0 }));
                }
                ui.separator();

                let mut current_index = module_res.current_material;
                ui.text_edit_singleline(
                    &mut module_res
                        .module
                        .materials
                        .get_mut(&current_index)
                        .unwrap()
                        .0,
                );

                let current_label = module_res.module.materials[&current_index].0.clone();
                egui::ComboBox::from_label("Material")
                    .selected_text(current_label)
                    .show_ui(ui, |ui| {
                        for (i, v) in module_res.module.materials.iter() {
                            ui.selectable_value(&mut current_index, *i, v.0.clone());
                        }
                    });
                module_res.current_material = current_index;

                if let MaterialDefinition::Color { .. } =
                    module_res.module.materials[&current_index].1
                {
                    if ui.button("Convert to PBR").clicked() {
                        module_res
                            .module
                            .materials
                            .get_mut(&current_index)
                            .unwrap()
                            .1 = default_pbr();
                    }
                } else if let MaterialDefinition::Pbr { .. } =
                    module_res.module.materials[&current_index].1
                {
                    if ui.button("Convert to Color").clicked() {
                        module_res
                            .module
                            .materials
                            .get_mut(&current_index)
                            .unwrap()
                            .1 = MaterialDefinition::Color { r: 0, g: 0, b: 0 };
                    }
                }

                match &mut module_res
                    .module
                    .materials
                    .get_mut(&current_index)
                    .unwrap()
                    .1
                {
                    MaterialDefinition::Color { r, g, b } => {
                        ui.label("RGB Solid Color");
                        let mut color = Color32::from_rgb(*r, *g, *b);
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        *r = color.r();
                        *g = color.g();
                        *b = color.b();
                    }
                    MaterialDefinition::Pbr {
                        display_color,
                        albedo,
                        normal_map,
                        occlusion,
                        metallic_roughness_texture,
                        emissive,
                        roughness,
                        metallic,
                    } => {
                        ui.label("Editor Display Color");
                        let mut color =
                            Color32::from_rgb(display_color.0, display_color.1, display_color.2);
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        display_color.0 = color.r();
                        display_color.1 = color.g();
                        display_color.2 = color.b();
                        ui.label("Base Color Texture Filename");
                        ui.text_edit_singleline(albedo);
                        ui.label("Normal Map Texture Filename (empty for none)");
                        ui.text_edit_singleline(normal_map);
                        ui.label("Occlusion Map Texture Filename (empty for none)");
                        ui.text_edit_singleline(occlusion);
                        ui.label("Metallic/Roughness Texture Filename (empty for none)");
                        ui.text_edit_singleline(metallic_roughness_texture);
                        ui.label("Emissive Texture Filename (empty for none)");
                        ui.text_edit_singleline(emissive);
                        ui.label("Rougness Number");
                        egui::Slider::new(roughness, 0.089..=1.0).ui(ui);
                        ui.label("Metallic Number");
                        egui::Slider::new(metallic, 0.0..=1.0).ui(ui);
                    }
                }
            });
    }

    if module_res.show_maps {
        egui::Window::new("Maps in Module")
            .title_bar(true)
            .show(egui_context.ctx(), |ui| {
                ui.label("New Map Name");
                ui.text_edit_singleline(&mut module_res.new_map.name);
                ui.label("Width");
                egui::Slider::new(&mut module_res.new_map.size.0, 1..=64).ui(ui);
                ui.label("Height");
                egui::Slider::new(&mut module_res.new_map.size.1, 1..=64).ui(ui);
                if ui.button("Create Map").clicked() {
                    let id = module_res.module.next_map_index;
                    module_res.module.next_map_index += 1;
                    let m = module_res.new_map.clone();
                    module_res.module.maps.insert(id, m);
                }

                ui.separator();
                if module_res.module.maps.is_empty() {
                    ui.label("There are no maps");
                } else {
                    let mut new_map: Option<usize> = None;
                    for (k, v) in module_res.module.maps.iter() {
                        if ui.button(&v.name).clicked() {
                            new_map = Some(*k);
                        }
                    }
                    if new_map.is_some() {
                        module_res.editing_map = new_map;
                    }
                }
            });
    }

    if let Some(map_id) = module_res.editing_map {
        let mut es = module_res.editor_settings.clone();
        MapEditor::render_in_module(egui_context.ctx(), &mut es, &mut module_res.module, map_id);
        module_res.editor_settings = es;
    }

    if module_res.show_events {
        egui::Window::new("Events")
            .title_bar(true)
            .show(egui_context.ctx(), |ui| {
                ui.label("New Event Tag");
                ui.text_edit_singleline(&mut module_res.new_event_tag);
                if ui.button("Add Event").clicked() {
                    let new_event = GameEvent{
                        tag: module_res.new_event_tag.clone(),
                        steps: Vec::new()
                    };
                    module_res.module.events.push(new_event);
                }
                ui.separator();
                let mut edit_event = None;
                for e in module_res.module.events.iter() {
                    if ui.button(&e.tag).clicked() {
                        edit_event = Some(e.tag.clone());
                    }
                }
                if edit_event.is_some() {
                    module_res.editing_event = edit_event;
                }
            });
    }

    if module_res.editing_event.is_some() {
        let tag = module_res.editing_event.clone().unwrap();
        let mut next_step = module_res.new_event_step;
        if let Some(event) = module_res.module.events.iter_mut().find(|e| e.tag.eq(&tag)) {
            egui::Window::new(format!("Event: {}", tag))
                .title_bar(true)
                .resizable(true)
                .show(egui_context.ctx(), |ui| {
                    ui.text_edit_singleline(&mut event.tag);

                    egui::ComboBox::from_label("New Step").show_ui(ui, |ui| {
                        ui.selectable_value(&mut next_step, EventPicker::LogText, "Log");
                        ui.selectable_value(&mut next_step, EventPicker::CallEvent, "Call");
                    });

                    if ui.button("Add Step").clicked() {
                        match next_step {
                            EventPicker::LogText => {
                                event.steps.push(GameEventStep::LogText("Hello".to_string()));
                            }
                            EventPicker::CallEvent => {
                                event.steps.push(GameEventStep::CallEvent(String::new()));
                            }
                        }
                    }

                    // List steps
                    for (line, e) in event.steps.iter_mut().enumerate() {
                        match e {
                            GameEventStep::LogText(s) => {
                                ui.label(format!("{} : Log Text", line));
                                ui.text_edit_singleline(s);
                            }
                            GameEventStep::CallEvent(tag) => {
                                ui.label(format!("{} : Call Event", line));
                                ui.text_edit_singleline(tag);
                            }
                        }
                    }
                });
        }
        module_res.new_event_step = next_step;
    }
}

pub fn resume_module_editor(mut commands: Commands, startup: Res<ModuleSelector>) {
    if let Some(filename) = &startup.filename {
        commands.insert_resource(ModuleResource {
            module: Module::load(&filename),
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
