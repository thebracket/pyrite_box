use super::ModuleResource;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub fn module_info(egui_context: &EguiContext, module_res: &mut ModuleResource) {
    if module_res.show_info {
        egui::Window::new("Module Editor")
            .auto_sized()
            .title_bar(true)
            .show(egui_context.ctx(), |ui| {
                ui.label("Module Name");
                ui.text_edit_singleline(&mut module_res.module.name);
                ui.label("Module Description");
                ui.text_edit_multiline(&mut module_res.module.description);
                ui.label("Author");
                ui.text_edit_multiline(&mut module_res.module.author);
                ui.label("Event Tag - On Start");
                ui.text_edit_singleline(&mut module_res.module.module_start_event);
                ui.label("Base Path");
                ui.text_edit_singleline(&mut module_res.module.base_path);
            });
    }
}
