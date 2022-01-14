use crate::{
    modules::game_events::{event_runner, event_triggers},
    state::AppState,
};
use bevy::prelude::{Plugin, SystemSet};
mod asset_loader;
mod game_log;
mod map_wander;
mod player_movement;
mod sprites;
use asset_loader::*;
pub use game_log::*;
pub use map_wander::*;
pub use player_movement::*;
pub use sprites::*;

pub struct MapWanderPlugin;

impl Plugin for MapWanderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // Map Wander Loader
            .add_system_set(
                SystemSet::on_enter(AppState::MapWanderLoader).with_system(start_asset_loader),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MapWanderLoader).with_system(asset_loader),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MapWanderLoader).with_system(finish_asset_loader),
            )
            // Map Wander
            .add_system_set(SystemSet::on_enter(AppState::MapWander).with_system(resume_map_wander))
            .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(map_wander))
            .add_system_set(
                SystemSet::on_update(AppState::MapWander).with_system(map_wander_rebuild),
            )
            .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(display_game_log))
            .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(event_triggers))
            .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(event_runner))
            .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(player_move))
            .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(region_sprites))
            .add_system_set(SystemSet::on_update(AppState::MapWander).with_system(billboarding))
            .add_system_set(SystemSet::on_exit(AppState::MapWander).with_system(exit_map_wander));
    }
}
