use bevy::prelude::*;

mod input;
mod ui;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui::create_ui);
        app.add_systems(Update, input::handle_selected_boxes);
        //app.add_observer(input::select_text_input_box);
        //app.add_observer(input::unselect_text_input_box);
    }
}

// Steal from exploration impl, or create a new one
// Step one: choose ui framework (if wanted)
