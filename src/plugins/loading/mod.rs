mod loading;
use crate::AppState;
use bevy::prelude::{Plugin, SystemSet};
pub use loading::*;
mod ui_assets;
pub use ui_assets::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_ui)
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_screen))
            .add_system_set(
                SystemSet::on_enter(AppState::Loading).with_system(resume_loading_screen),
            )
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(exit_loading));
    }
}
