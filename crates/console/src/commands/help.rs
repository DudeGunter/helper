use crate::custom_message;
use crate::prelude::*;
use bevy::prelude::*;

pub fn help(In(arguments): In<String>, mut commands: Commands, config: Res<ConsoleConfig>) {
    info!("Running command help with the arguments: {}", arguments);
    let commands = config.get_commands();

    for command in commands {
        commands.trigger(custom_message![(*command)]);
        if let Some(metadata) = config.get_metadata(command) {
            info!("Description: {}", metadata.description);
            info!("Usage: {}", metadata.usage);
        }
    }
}
