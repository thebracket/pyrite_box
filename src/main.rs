use bevy::prelude::*;
mod region;
use plugins::PyritePlugin;
use state::AppState;
mod modules;
mod plugins;
mod state;
mod utils;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pyrite Box".to_string(),
            width: 1280.0,
            height: 1024.0,
            //vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PyritePlugin)
        .run();
}
