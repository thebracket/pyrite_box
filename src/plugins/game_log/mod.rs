use crate::state::AppState;
use bevy::prelude::{Commands, Plugin, SystemSet};
mod log_resource;
pub use log_resource::*;
mod log_manager;
pub use log_manager::*;
mod messages;
pub use messages::*;

pub struct GameLogPlugin;

impl Plugin for GameLogPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<LogMessage>()
            .add_startup_system(enable_log_resource)
            .add_system_set(SystemSet::on_enter(AppState::PlayRegion).with_system(create_log_panel))
            .add_system_set(
                SystemSet::on_update(AppState::PlayRegion)
                    .with_system(update_game_log)
                    .with_system(animate_game_log),
            );
    }
}

fn enable_log_resource(mut commands: Commands) {
    commands.insert_resource(log_resource::GameLog::new());
}
