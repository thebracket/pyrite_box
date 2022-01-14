use crate::{
    modules::{list_available_modules, ModuleHeader},
    plugins::CharacterHeader,
};

pub struct AvailableModules {
    pub modules: Vec<ModuleHeader>,
    pub characters: Vec<CharacterHeader>,
}

impl AvailableModules {
    pub fn new() -> Self {
        Self {
            modules: list_available_modules(),
            characters: CharacterHeader::scan_available(),
        }
    }
}
