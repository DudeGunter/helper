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
        let message = commands
            .spawn((ConsoleMessage, Text::default(), Node::default()))
            .with_children(|parent| {
                for span in trigger.spans.iter() {
                    parent.spawn(span.bundle());
                }
            })
            .id();
        commands.entity(entity).add_child(message);
    }
}

#[derive(Event)]
pub struct CustomMessage {
    pub spans: Vec<Span>,
}

impl CustomMessage {
    fn simple<S: Into<String>>(text: S) -> Self {
        Self {
            spans: vec![Span::from(text.into())],
        }
    }
}

pub struct Span {
    text: String,
    color: Color,
}

impl From<String> for Span {
    fn from(text: String) -> Self {
        Self {
            text,
            color: DEFAULT_CONSOLE_FONT_COLOR,
        }
    }
}

impl From<(String, Color)> for Span {
    fn from(tuple: (String, Color)) -> Self {
        Self {
            text: tuple.0,
            color: tuple.1,
        }
    }
}

impl Span {
    fn new<S: Into<String>, C: Into<Color>>(text: S, color: C) -> Self {
        Self {
            text: text.into(),
            color: color.into(),
        }
    }

    fn bundle(&self) -> impl Bundle {
        (
            TextSpan::new(self.text.clone()),
            TextColor(self.color),
            TextFont {
                font_size: crate::ui::CONSOLE_FONT_SIZE,
                ..default()
            },
        )
    }
}

// Temporarily replace the macro with this to see the expansion
#[macro_export]
macro_rules! custom_message {
    ($($item:tt),* $(,)?) => {{
        // Debug: print the type
        let spans_vec = vec![
            $(custom_message!(@span $item)),*
        ];
        let result = $crate::message::CustomMessage {
            spans: spans_vec
        };
        result
    }};

    (@span ($text:expr, $color:expr)) => {
        $crate::message::Span::new($text, $color)
    };

    (@span ($text:expr)) => {
        $crate::message::Span::from($text)
    };

    (@span $text:expr) => {
        $crate::message::Span::from($text)
    };
}
