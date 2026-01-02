use bevy::{log::LogPlugin, prelude::*};
use console::ConsolePlugin;
use purple::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                custom_layer: console::logging::custom_log_layer,
                ..default()
            }),
            ConsolePlugin,
            PurplePlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    info!("Basic example, spawning scene");
    commands.spawn((
        Camera3d::default(),
        Transform::default().looking_at(Vec3::new(10.0, 0.0, 0.0), Vec3::Y),
    ));
    commands.spawn((DebugCube, Transform::from_xyz(10.0, 0.0, 0.0)));
}
