use bevy::prelude::*;

pub struct ConsoleUI;

pub fn create_ui(mut commands: Commands) {
    commands.spawn((ConsoleUI, Node::default()));
}
