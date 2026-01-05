use crate::prelude::*;
use bevy::prelude::*;

pub fn help(In(_arguments): In<String>, config: Res<ConsoleConfig>) {
    let console_commands = config.get_commands();

    for command in console_commands {
        simple!("{}", command);
        if let Some(metadata) = config.get_metadata(command) {
            simple!(" >Description: {}", metadata.description);
            simple!(" >Usage: {}", metadata.usage);
        }
    }
}
