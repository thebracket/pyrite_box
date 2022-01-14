#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Loading,
    MainMenu,
    ModuleEditor,
    ModuleAssetLoader, // Loading screen for the map module
    PlayRegion,       // Test mode for the map
    Battle,
    CharacterGeneration,
}
