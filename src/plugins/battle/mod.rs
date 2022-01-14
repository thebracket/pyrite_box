use crate::state::AppState;
mod battle_map;
mod battle_runner;
mod battle_tile;
use battle_runner::*;
use bevy::prelude::{App, Plugin, SystemSet};
mod components;
pub use components::BattleComponent;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Battle).with_system(start_battle));
    }
}
