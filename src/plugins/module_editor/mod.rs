use bevy::prelude::{Plugin, SystemSet};

use crate::AppState;

mod editor;
mod events;
mod maps;
mod materials;
mod menu;
mod module_info;
mod module_resource;
use editor::*;

pub struct ModuleEditorPlugin;

impl Plugin for ModuleEditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_update(AppState::ModuleEditor).with_system(module_editor), //.with_system(texture_mode_system)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::ModuleEditor).with_system(resume_module_editor),
        )
        .add_system_set(SystemSet::on_exit(AppState::ModuleEditor).with_system(exit_module_editor));
    }
}
