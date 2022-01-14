use bevy_egui::egui;
use bevy_egui::EguiContext;

use super::module_resource::ModuleResource;

pub fn editor_menu(egui_context: &EguiContext, module_res: &mut ModuleResource) {
    egui::TopBottomPanel::top("menu_bar").show(egui_context.ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Module Editor", |ui| {
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
}
