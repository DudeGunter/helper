use bevy::prelude::*;

pub fn help(In(arguments): In<String>) {
    info!("Running command help with the arguments: {}", arguments);
}
