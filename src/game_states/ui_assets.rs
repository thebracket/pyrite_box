use bevy::prelude::*;
use bevy_egui::{
    egui::{FontDefinitions, FontFamily, FontData},
    EguiContext,
};

pub struct UiAssets {
    pub title: Handle<Texture>,
    pub title_mat: Handle<ColorMaterial>,
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    egui_context: ResMut<EguiContext>,
) {
    let mut fd = FontDefinitions {
        ..Default::default()
    };
    fd.font_data.insert(
        "Olde".to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/fonts/SdThoseGoodTimesOfLife-B1An.ttf"
        ),
    ));
    let ff = fd
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap();
    ff.clear();
    ff.push("Olde".to_string());
    egui_context.ctx().set_fonts(fd);
    let title = asset_server.load("images/pyrite.png");
    let title_mat = materials.add(title.clone().into());
    commands.insert_resource(UiAssets {
        title: title.clone(),
        title_mat: title_mat.clone(),
    });
}
