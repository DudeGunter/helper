use bevy::prelude::*;

pub struct PurplePlugin;

impl Plugin for PurplePlugin {
    fn build(&self, app: &mut App) {
        // Incase its added in startup and the scheduler is annoying
        app.add_systems(PreStartup, setup_debug_material);
        app.add_observer(spawn_debug_cube);
    }
}

#[derive(Resource)]
pub struct DebugMaterial(pub Handle<StandardMaterial>);

pub fn setup_debug_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial::from_color(Color::srgb(0.5, 0.0, 0.5)));
    commands.insert_resource(DebugMaterial(material));
}

/// Spawn me and my mesh and material will be added automatically,
/// Make sure you have the plugin added.
#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
pub struct DebugCube;

pub fn spawn_debug_cube(
    trigger: On<Add, DebugCube>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<DebugMaterial>,
) {
    let mesh = meshes.add(Cuboid::default());
    commands
        .entity(trigger.entity)
        .insert((Mesh3d(mesh), MeshMaterial3d(materials.0.clone())));
}
