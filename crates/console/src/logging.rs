use bevy::{color::palettes::css::*, log::BoxedLayer, prelude::*};
use crossbeam::channel::{Receiver, Sender};
use tracing::Level;

use bevy::log::tracing_subscriber;

/// A function that implements the log reading functionality for the
/// developer console via [`LogPlugin::custom_layer`](bevy::log::LogPlugin::custom_layer).
/// ```rust
/// # use bevy::prelude::*;
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins.set(LogPlugin {
///             custom_layer: custom_log_layer, // The function
///             ..Default::default()
///         }))
///         .add_plugin(ConsolePlugin)
///         .run();
/// }
/// ```
pub fn custom_log_layer(app: &mut App) -> Option<BoxedLayer> {
    Some(Box::new(create_custom_log_layer(app)))
}

fn create_custom_log_layer(app: &mut App) -> BoxedLayer {
    let (sender, receiver) = crossbeam::channel::unbounded();
    app.insert_resource(TracingReceiver(receiver));

    Box::new(ConsoleLayer(sender))
}

#[derive(Debug)]
pub struct TraceMessage {
    pub level: Level,
    pub target: String,
    pub message: String,
}

#[derive(Resource, Deref, DerefMut)]
pub struct TracingReceiver(pub Receiver<TraceMessage>);

struct ConsoleLayer(Sender<TraceMessage>);

impl<S> tracing_subscriber::Layer<S> for ConsoleLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = ConsoleVisitor::default();
        event.record(&mut visitor);

        let level = *event.metadata().level();
        let target = event.metadata().target();

        let _ = self.0.send(TraceMessage {
            level: level,
            target: target.to_string(),
            message: visitor.message,
        });
    }
}

#[derive(Default)]
struct ConsoleVisitor {
    message: String,
}

impl tracing::field::Visit for ConsoleVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value).trim_matches('"').to_string();
        }
    }
}

pub trait GetColor {
    fn get_color(&self) -> Color;
}

impl GetColor for Level {
    fn get_color(&self) -> Color {
        Color::from(match *self {
            Level::ERROR => RED,
            Level::WARN => YELLOW,
            Level::INFO => GREEN,
            Level::DEBUG => GRAY,
            Level::TRACE => DARK_GRAY,
        })
    }
}
