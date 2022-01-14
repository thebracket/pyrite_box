use crate::modules::Module;

/// Stores main-menu module and party selection. This is then
/// transferred to gameplay creation, editor creation, etc.
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
