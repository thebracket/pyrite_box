use crate::module::default_pbr;
use crate::module::MaterialDefinition;
use bevy_egui::egui;
use bevy_egui::egui::Color32;
use bevy_egui::egui::Widget;
use bevy_egui::EguiContext;

use super::ModuleResource;

pub fn material_editor(egui_context: &EguiContext, module_res: &mut ModuleResource) {
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
                    module_res.module.materials.insert(
                        id,
                        (
                            name,
                            MaterialDefinition::Color { r: 0, g: 0, b: 0 },
                            "black.ron".to_string(),
                        ),
                    );
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
}
