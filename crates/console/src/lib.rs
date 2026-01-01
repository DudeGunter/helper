use bevy::prelude::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, || info!("Console impl wip."));
    }
}

// Steal from exploration impl, or create a new one
// Step one: choose ui framework (if wanted)
