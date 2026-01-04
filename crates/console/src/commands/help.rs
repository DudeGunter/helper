use crate::prelude::*;
use bevy::prelude::*;

pub fn help(In(arguments): In<String>, config: Res<ConsoleConfig>) {
    info!("Running command help with the arguments: {}", arguments);
    let commands = config.get_commands();

    for command in commands {
        info!("Command: {}", command);
        if let Some(metadata) = config.get_metadata(command) {
            info!("Description: {}", metadata.description);
            info!("Usage: {}", metadata.usage);
        }
    }
}
