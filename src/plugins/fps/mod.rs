use bevy::prelude::Plugin;
mod fps;
pub use fps::*;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_fps)
            .add_system(fps_update_system);
    }
}
