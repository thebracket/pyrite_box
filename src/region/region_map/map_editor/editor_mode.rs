#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapEditorMode {
    Walls,
    Floor,
    Ceiling,
    Start,
    Opening,
}
