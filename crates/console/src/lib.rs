use bevy::{ecs::system::SystemId, platform::collections::HashMap, prelude::*};

use crate::commands::{CommandMetadata, Commands};

mod commands;
mod input;
mod logging;
mod message;
mod parser;
mod ui;
// todo! the file structure could be more conservative

/// The Plugin that implements the log reading functionality for the
/// developer console via [`LogPlugin::custom_layer`](bevy::log::LogPlugin::custom_layer).
/// ```rust
/// # use bevy::prelude::*;
/// // The Default plugin - I assume - isn't actually needed
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins.set(LogPlugin {
///             custom_layer: custom_log_layer, // The function
///             ..default()
///         }))
///         .add_plugin(ConsolePlugin) // The plugin
///         .run();
/// }
/// ```
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleConfig>();
        app.add_systems(Startup, (ui::create_ui, commands::collect_commands));
        app.add_systems(
            Update,
            (
                input::handle_selected_boxes,
                message::receive_traced_message,
                open_close_console,
            ),
        );
        app.add_observer(commands::run_submitted_commands);
        app.add_observer(message::handle_custom_messages);

        // Default commands
        app.insert_command_with_name(
            "help",
            CommandMetadata {
                description: "Display helpful information about different commands".to_string(),
                usage: "help".to_string(),
            },
            commands::help::help,
        );
        app.insert_command("clear", commands::clear::clear);
        app.insert_command("quit", commands::quit::quit);
    }
}

/// Console configuration resource
/// Houses the command references
#[derive(Resource)]
pub struct ConsoleConfig {
    pub open_close_key: KeyCode,
    // this shouldn't be edited (probably idk) manually
    pub(crate) commands: HashMap<String, (Option<CommandMetadata>, SystemId<In<String>>)>,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            open_close_key: KeyCode::KeyT,
            commands: HashMap::default(),
        }
    }
}

impl ConsoleConfig {
    pub fn get_commands(&self) -> Vec<&String> {
        self.commands.keys().collect()
    }

    pub fn get_metadata(&self, command: &str) -> Option<CommandMetadata> {
        self.commands
            .get(command)
            .map(|(metadata, _)| metadata)
            .unwrap_or(&None)
            .clone()
    }
}

pub fn open_close_console(
    config: Res<ConsoleConfig>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<crate::ui::ConsoleUI>>,
    select_check: Query<Entity, With<crate::input::SelectedBox>>,
) {
    if !select_check.is_empty() {
        // If a box is selected, don't open/close the console (key protection)
        return;
    }

    if input.just_pressed(config.open_close_key) {
        if let Ok(mut visibility) = query.single_mut() {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Visible,
                Visibility::Visible => Visibility::Hidden,
                _ => Visibility::Hidden,
            };
        }
    }
}

pub mod prelude {
    pub use crate::ConsoleConfig;
    pub use crate::ConsolePlugin;
    pub use crate::logging::custom_log_layer;
    pub use crate::simple;
    pub use bevy::log::LogPlugin;
}
