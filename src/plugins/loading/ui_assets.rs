use bevy::prelude::*;
use bevy_egui::{
    egui::{FontData, FontDefinitions, FontFamily},
    EguiContext,
};

pub struct UiFonts {
    pub game_font: Handle<Font>,
}

pub fn setup_ui(
    egui_context: ResMut<EguiContext>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut fd = FontDefinitions {
        ..Default::default()
    };
    fd.font_data.insert(
        "Olde".to_owned(),
        FontData::from_static(include_bytes!(
            "../../../assets/fonts/SdThoseGoodTimesOfLife-B1An.ttf"
        )),
    );
    /*let ff = fd
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap();
    ff.clear();
    ff.push("Olde".to_string());
    egui_context.ctx().set_fonts(fd);*/

    let game_font = asset_server.load("fonts/SdThoseGoodTimesOfLife-B1An.ttf");
    commands.insert_resource(UiFonts { game_font });
}
