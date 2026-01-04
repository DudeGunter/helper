use crate::message::CustomMessage;
use crate::prelude::*;
use bevy::prelude::*;

pub fn help(In(_arguments): In<String>, mut commands: Commands, config: Res<ConsoleConfig>) {
    let console_commands = config.get_commands();

    for command in console_commands {
        commands.trigger(CustomMessage::simple(command));
        info!(custom = true, "hello");
        if let Some(metadata) = config.get_metadata(command) {
            commands.trigger(CustomMessage::simple(format!(
                " >Description: {}",
                metadata.description
            )));
            commands.trigger(CustomMessage::simple(format!(
                " >Usage: {}",
                metadata.usage
            )));
        }
    }
}
