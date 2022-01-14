mod main_menu;
use bevy::prelude::{Plugin, SystemSet};
pub use main_menu::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(main_menu))
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(resume_main_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(exit_main_menu));
    }
}
