use crate::modules::game_events::InputChoice;

pub struct WanderInput {
    pub title: String,
    pub message: String,
    pub blocked: bool,
    pub options: Vec<InputChoice>,
    pub result: Option<usize>,
    pub portrait: Option<String>,
}
