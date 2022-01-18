mod fps;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::Plugin};
use bevy_egui::EguiPlugin;
pub use fps::*;
mod loading;
pub use loading::*;
mod main_menu;
pub use main_menu::*;
mod character_generator;
pub use character_generator::*;
mod module_editor;
pub use module_editor::*;
mod battle;
pub use battle::*;
mod play_region;
use crate::{modules::game_events::TriggerEvent, AppState};
pub use play_region::*;
mod game_log;
pub use game_log::*;

pub struct PyritePlugin;

impl Plugin for PyritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state(AppState::Loading)
            .add_event::<TriggerEvent>()
            .add_event::<PlayerMoveRequest>()
            .add_event::<SpriteRequest>()
            .add_event::<MoveOccurred>()
            .add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(FpsPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(CharacterGeneratorPlugin)
            .add_plugin(ModuleEditorPlugin)
            .add_plugin(BattlePlugin)
            .add_plugin(MapWanderPlugin)
            .add_plugin(GameLogPlugin);
    }
}
