mod generator;
use bevy::prelude::{Plugin, SystemSet};
pub use generator::*;
mod character_header;
pub use character_header::*;

use crate::AppState;

pub struct CharacterGeneratorPlugin;

impl Plugin for CharacterGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_update(AppState::CharacterGeneration).with_system(run_chargen),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::CharacterGeneration).with_system(start_chargen),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::CharacterGeneration).with_system(exit_chargen),
        );
    }
}
