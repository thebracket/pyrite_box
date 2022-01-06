use bevy::prelude::*;
use bevy_egui::{
    egui::{FontData, FontDefinitions, FontFamily},
    EguiContext,
};

pub fn setup_ui(
    egui_context: ResMut<EguiContext>,
) {
    let mut fd = FontDefinitions {
        ..Default::default()
    };
    fd.font_data.insert(
        "Olde".to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/fonts/SdThoseGoodTimesOfLife-B1An.ttf"
        )),
    );
    let ff = fd
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap();
    ff.clear();
    ff.push("Olde".to_string());
    egui_context.ctx().set_fonts(fd);
}
