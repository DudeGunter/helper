use crate::{ConsoleConfig, input::SubmittedText};
use bevy::{ecs::system::SystemId, prelude::*};

pub mod clear;
pub mod help;
pub mod quit;

#[derive(Component)]
pub struct CommandToCollect {
    name: String,
    metadata: Option<CommandMetadata>,
    system: SystemId<In<String>>,
}

#[derive(Reflect, Clone)]
pub struct CommandMetadata {
    pub description: String,
    // todo! decide wether to make this a struct with a patern of some sort with
    // integrated parsing (if that make sense) possible use clap?
    pub usage: String,
}

pub trait ConsoleCommands {
    #![allow(unused)]
    fn insert_command<S: Into<String>, M: 'static>(
        &mut self,
        name: S,
        system: impl IntoSystem<In<String>, (), M> + Send + Sync + 'static,
    ) -> &mut Self;

    fn insert_command_with_name<T: Into<String>, M: 'static>(
        &mut self,
        name: T,
        command: CommandMetadata,
        system: impl IntoSystem<In<String>, (), M> + Send + Sync + 'static,
    ) -> &mut Self;
}

impl ConsoleCommands for App {
    fn insert_command<S: Into<String>, M: 'static>(
        &mut self,
        name: S,
        system: impl IntoSystem<In<String>, (), M> + Send + Sync + 'static,
    ) -> &mut Self {
        let world = self.world_mut();
        let system = world.register_system(system);
        // Instead, we spawn a component to be collected on startup
        world.spawn(CommandToCollect {
            name: name.into(),
            metadata: None,
            system,
        });
        self
    }

    fn insert_command_with_name<T: Into<String>, M: 'static>(
        &mut self,
        name: T,
        command: CommandMetadata,
        system: impl IntoSystem<In<String>, (), M> + Send + Sync + 'static,
    ) -> &mut Self {
        let world = self.world_mut();
        let system = world.register_system(system);
        // Instead, we spawn a component to be collected on startup
        world.spawn(CommandToCollect {
            name: name.into(),
            metadata: Some(command),
            system,
        });
        self
    }
}

pub fn collect_commands(
    mut commands: bevy::prelude::Commands,
    mut console: ResMut<ConsoleConfig>,
    query: Query<(Entity, &CommandToCollect)>,
) {
    for (entity, command) in query.iter() {
        commands.entity(entity).despawn();
        console.commands.insert(
            command.name.clone(),
            (command.metadata.clone(), command.system),
        );
    }
}

pub fn run_submitted_commands(
    on: On<SubmittedText>,
    mut commands: bevy::prelude::Commands,
    console: Res<ConsoleConfig>,
) {
    let (command_name, arguments) = on.text.split_once(' ').unwrap_or((on.text.as_str(), ""));
    if let Some((_command, system)) = console.commands.get(command_name) {
        commands.run_system_with(*system, arguments.to_string());
    } else {
        warn!("Command not found: {}", command_name);
    }
}
