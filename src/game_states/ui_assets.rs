use bevy::prelude::*;

pub struct UiAssets {
    pub title : Handle<Texture>,
    pub title_mat : Handle<ColorMaterial>,
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let title = asset_server.load("images/pyrite.png");
    let title_mat = materials.add(title.clone().into());
    commands.insert_resource(UiAssets{
        title : title.clone(),
        title_mat : title_mat.clone(),
    });
}