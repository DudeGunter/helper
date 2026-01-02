use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender};

mod input;
mod ui;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui::create_ui);
        app.add_systems(Update, input::handle_selected_boxes);
    }
}

struct ConsoleLayer(Sender<String>);

struct ConsoleReciever(Receiver<String>);
