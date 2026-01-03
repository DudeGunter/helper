use bevy::prelude::*;

use crate::input::SubmittedText;

mod input;
pub mod logging;
mod message;
mod ui;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui::create_ui);
        app.add_systems(
            Update,
            (
                input::handle_selected_boxes,
                message::receive_traced_message,
            ),
        );
        app.add_observer(|on_submit: On<SubmittedText>| info!("{}", on_submit.text));
    }
}
