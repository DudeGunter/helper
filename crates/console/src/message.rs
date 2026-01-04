use bevy::{
    color::palettes::{css::*, tailwind::*},
    prelude::*,
};

use crate::{
    logging::{GetColor, TracingReceiver},
    ui::MessageContainer,
};

#[derive(Component)]
pub struct ConsoleMessage;

pub fn receive_traced_message(
    mut commands: Commands,
    container: Query<Entity, With<MessageContainer>>,
    traced_messages: ResMut<TracingReceiver>,
) {
    if let Ok(entity) = container.single_inner() {
        let mut new_messages: Vec<Entity> = Vec::new();
        while let Ok(trace) = traced_messages.try_recv() {
            let info = span(trace.level.to_string() + " ", trace.level.get_color());
            let path = span(trace.target + ": ", BLUE_600);
            let message = span(trace.message, WHITE_SMOKE);
            let message = commands
                .spawn((
                    ConsoleMessage,
                    Text::default(),
                    Node::default(),
                    children![info, path, message],
                ))
                .id();
            new_messages.push(message);
        }
        commands.entity(entity).add_children(&new_messages);
    }
}

pub fn span<S: Into<String>, C: Into<Color>>(string: S, color: C) -> impl Bundle {
    (
        TextSpan::new(string.into()),
        TextColor(color.into()),
        TextFont {
            font_size: crate::ui::CONSOLE_FONT_SIZE,
            ..default()
        },
    )
}
