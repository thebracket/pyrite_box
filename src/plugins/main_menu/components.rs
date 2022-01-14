use bevy::prelude::Component;

/// Tag component indicating that the associated entity is part of the
/// main menu user interface, and should be removed when the menu
/// ceases operation.
#[derive(Component)]
pub struct MainMenuUi;
