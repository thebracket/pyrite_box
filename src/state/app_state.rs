#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Loading,
    MainMenu,
    ModuleEditor,
    MapWanderLoader, // Loading screen for the map module
    MapWander,       // Test mode for the map
    Battle,
    CharacterGeneration,
}
