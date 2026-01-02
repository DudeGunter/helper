//! Custom [LogPlugin](bevy::log::LogPlugin) functionality.

use bevy::log::BoxedLayer;
use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender};

use bevy::{
    log::{tracing_subscriber, tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt},
    prelude::*,
};

/// A function that implements the log reading functionality for the
/// developer console via [`LogPlugin::custom_layer`](bevy::log::LogPlugin::custom_layer).
pub fn custom_log_layer(app: &mut App) -> Option<BoxedLayer> {
    Some(Box::new(create_custom_log_layer(app)))
}

fn create_custom_log_layer(app: &mut App) -> BoxedLayer {
    let (sender, receiver) = crossbeam::channel::unbounded();
    app.insert_resource(ConsoleReciever(receiver));

    Box::new(ConsoleLayer(sender))
}

#[derive(Resource)]
pub struct ConsoleReciever(Receiver<String>);

struct ConsoleLayer(Sender<String>);

impl<S> tracing_subscriber::Layer<S> for ConsoleLayer
where
    S: tracing::Subscriber,
{
    fn on_event(&self, event: &tracing::Event, _: tracing_subscriber::layer::Context<S>) {
        let mut visitor = ConsoleVisitor::default();
        event.record(&mut visitor);
        let msg = format!("[{}] {}", event.metadata().level(), visitor.message);
        let _ = self.0.send(msg);
    }
}

#[derive(Default)]
struct ConsoleVisitor {
    message: String,
}

impl tracing::field::Visit for ConsoleVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        // This gets called for each field in the event
        if field.name() == "message" {
            // Found the message field, extract it
            self.message = format!("{:?}", value);
        }
    }
}
