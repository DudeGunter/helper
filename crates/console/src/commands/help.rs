use crate::prelude::*;
use bevy::prelude::*;

pub fn help(In(_arguments): In<String>, config: Res<ConsoleConfig>) {
    for command in config.get_commands() {
        simple!("{}", command);
        if let Some(metadata) = config.get_metadata(command) {
            simple!(" >Description: {}", metadata.description);
            simple!(" >Usage: {}", metadata.usage);
        }
    }
}
