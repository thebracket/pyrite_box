use super::ModuleResource;
use bevy_egui::egui;
use bevy_egui::egui::Widget;
use bevy_egui::EguiContext;

pub fn maps(egui_context: &EguiContext, module_res: &mut ModuleResource) {
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
}
