use bevy::prelude::*;

use crate::commands::Commands;

mod commands;
mod input;
mod logging;
mod message;
mod parser;
mod ui;

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
        app.init_resource::<commands::ConsoleCommands>();
        app.add_systems(Startup, (ui::create_ui, commands::collect_commands));
        app.add_systems(
            Update,
            (
                input::handle_selected_boxes,
                message::receive_traced_message,
            ),
        );
        app.add_observer(commands::run_submitted_commands);

        // Default commands
        app.insert_command(
            commands::Command {
                callable_name: "help".to_string(),
                description: "Display help information".to_string(),
            },
            commands::help::help,
        );
    }
}

pub mod prelude {
    pub use crate::ConsolePlugin;
    pub use crate::logging::custom_log_layer;
    pub use bevy::log::LogPlugin;
}
