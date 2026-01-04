use bevy::platform::collections::hash_map::HashMap;
use bevy::{ecs::system::SystemId, prelude::*};

use crate::input::SubmittedText;

pub mod help;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ConsoleCommands(HashMap<String, (Command, SystemId<In<String>>)>);

impl ConsoleCommands {
    pub fn insert(&mut self, command: Command, system: SystemId<In<String>>) {
        self.0
            .insert(command.callable_name.clone(), (command, system));
    }
}

#[derive(Component)]
pub struct CommandToCollect {
    command: Command,
    system: SystemId<In<String>>,
}

#[derive(Reflect, Clone)]
pub struct Command {
    pub callable_name: String,
    pub description: String,
}

pub trait Commands {
    fn insert_command<M: 'static>(
        &mut self,
        command: Command,
        system: impl IntoSystem<In<String>, (), M> + Send + Sync + 'static,
    );
}

impl Commands for App {
    fn insert_command<M: 'static>(
        &mut self,
        command: Command,
        system: impl IntoSystem<In<String>, (), M> + Send + Sync + 'static,
    ) {
        let world = self.world_mut();
        let system = world.register_system(system);
        // Instead, we spawn a component to be collected on startup
        world.spawn(CommandToCollect { command, system });
    }
}

pub fn collect_commands(
    mut commands: bevy::prelude::Commands,
    mut console: ResMut<ConsoleCommands>,
    query: Query<(Entity, &CommandToCollect)>,
) {
    for (entity, command) in query.iter() {
        commands.entity(entity).despawn();
        console.insert(command.command.clone(), command.system);
    }
}

pub fn run_submitted_commands(
    on: On<SubmittedText>,
    mut commands: bevy::prelude::Commands,
    console: Res<ConsoleCommands>,
) {
    let (command_name, arguments) = on.text.split_once(' ').unwrap_or((on.text.as_str(), ""));
    if let Some((_command, system)) = console.0.get(command_name) {
        info!(
            "Running command {} with the arguments: {}",
            command_name, arguments
        );
        commands.run_system_with(*system, arguments.to_string());
    } else {
        error!("Command not found: {}", command_name);
    }
}
