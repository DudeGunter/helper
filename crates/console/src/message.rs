use crate::{
    logging::{GetColor, TracingReceiver},
    ui::MessageContainer,
};
use bevy::{
    color::palettes::{css::*, tailwind::*},
    prelude::*,
};

pub const DEFAULT_CONSOLE_FONT_COLOR: Color = Color::Srgba(WHITE_SMOKE);

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
            if !trace.custom {
                let time = trace.time.time().to_string(); // todo! (not really) this formatting could be done better via chronos
                let formatted_time = &time[..time.len() - 7];
                let time = span(formatted_time.to_owned() + " ", GRAY_300);
                let info = span(trace.level.to_string() + " ", trace.level.get_color());
                let path = span(trace.target + ": ", BLUE_600);
                let message = span(trace.message, WHITE_SMOKE);
                let message = commands
                    .spawn((
                        ConsoleMessage,
                        Text::default(),
                        Node::default(),
                        children![time, info, path, message],
                    ))
                    .id();
                new_messages.push(message);
            } else {
                let message = commands
                    .spawn((
                        (ConsoleMessage, Text::from(trace.message)),
                        TextColor(DEFAULT_CONSOLE_FONT_COLOR),
                        TextFont {
                            font_size: crate::ui::CONSOLE_FONT_SIZE,
                            ..default()
                        },
                    ))
                    .id();
                new_messages.push(message);
            }
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

pub fn handle_custom_messages(
    trigger: On<CustomMessage>,
    mut commands: Commands,
    container: Query<Entity, With<MessageContainer>>,
) {
    if let Ok(entity) = container.single() {
        let message = trigger.bundle(&mut commands);
        commands.entity(entity).add_child(message);
    }
}

// High level control over the messages
#[derive(Event, Deref, DerefMut)]
pub struct CustomMessage(Vec<Span>);

impl CustomMessage {
    /// Redundent, use the simple! macro
    pub fn _simple<S: Into<String>>(text: S) -> Self {
        Self(vec![Span {
            text: text.into(),
            ..default()
        }])
    }

    pub fn bundle(&self, commands: &mut Commands) -> Entity {
        let mut span_entities: Vec<Entity> = Vec::new();
        for span in self.iter() {
            span_entities.push(commands.spawn(span.bundle()).id());
        }
        commands
            .spawn((ConsoleMessage, Text::default(), Node::default()))
            .add_children(span_entities.as_slice())
            .id()
    }
}

pub struct Span {
    text: String,
    color: Color,
}

impl Default for Span {
    fn default() -> Self {
        Self {
            text: String::new(),
            color: DEFAULT_CONSOLE_FONT_COLOR,
        }
    }
}

impl Span {
    pub fn bundle(&self) -> impl Bundle {
        (
            TextSpan::new(self.text.clone()),
            TextColor(self.color.clone()),
            TextFont {
                font_size: crate::ui::CONSOLE_FONT_SIZE,
                ..default()
            },
        )
    }
}

#[macro_export]
macro_rules! simple {
    ($msg:expr) => {
        tracing::info!(custom = true, $msg);
    };
    ($fmt:expr, $($arg:tt)*) => {
        tracing::info!(custom = true, $fmt, $($arg)*);
    };
}
