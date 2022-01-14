use crate::modules::Module;

pub struct ModuleSelector {
    pub module: Option<Module>,
    pub party: Vec<String>,
}

impl ModuleSelector {
    pub fn new() -> Self {
        Self {
            module: None,
            party: Vec::new(),
        }
    }
}
