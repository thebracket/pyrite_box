use bevy::prelude::Color;

pub enum LogMessage {
    Clear,
    AddLine { line: String, color: Color },
}
