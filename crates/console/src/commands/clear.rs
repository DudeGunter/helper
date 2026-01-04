use crate::message::ConsoleMessage;
use bevy::prelude::*;

pub fn clear(_: In<String>, mut commands: Commands, query: Query<Entity, With<ConsoleMessage>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
