use bevy::{
    ecs::{lifecycle::HookContext, system::SystemParam, world::DeferredWorld},
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

#[derive(Component, Reflect, Debug)]
#[component(on_add = Self::on_add)]
#[require(Node, Text)]
pub struct TextInputBox {
    clear_on_submit: bool,
}

impl Default for TextInputBox {
    fn default() -> Self {
        Self {
            clear_on_submit: true,
        }
    }
}

impl TextInputBox {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        world
            .commands()
            .entity(context.entity)
            .observe(select_text_input_box)
            .observe(unselect_text_input_box);
    }
}

/// Text submitted from TextInputBox
#[derive(Event, Clone, Debug)]
pub struct SubmittedText {
    pub text: String,
}

/// Marker component for selected text input boxes.
#[derive(Component, Reflect)]
pub struct SelectedBox;

pub fn select_text_input_box(trigger: On<Pointer<Over>>, mut commands: Commands) {
    commands.entity(trigger.entity).insert(SelectedBox);
}

pub fn unselect_text_input_box(trigger: On<Pointer<Out>>, mut commands: Commands) {
    commands.entity(trigger.entity).remove::<SelectedBox>();
}

pub fn handle_selected_boxes(
    mut commands: Commands,
    mut keyboard_input: MessageReader<KeyboardInput>,
    query: Query<(Entity, &TextInputBox, &mut Text), With<SelectedBox>>,
) {
    if let Ok((_entity, config, mut text)) = query.single_inner() {
        for input in keyboard_input.read() {
            if !input.state.is_pressed() {
                continue;
            }
            match (&input.logical_key, &input.text) {
                (Key::Enter, _) => {
                    commands.trigger(SubmittedText {
                        text: text.0.clone(),
                    });
                    if config.clear_on_submit {
                        text.clear();
                    }
                }
                (Key::Backspace, _) => {
                    if text.is_empty() {
                        continue;
                    }

                    text.pop();
                }
                (_, Some(inserted_text)) => {
                    if inserted_text.chars().all(is_printable_char) {
                        text.push_str(inserted_text);
                    }
                }
                _ => {}
            }
        }
    }
}

// this logic is taken from egui-winit:
// https://github.com/emilk/egui/blob/adfc0bebfc6be14cee2068dee758412a5e0648dc/crates/egui-winit/src/lib.rs#L1014-L1024
fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}

/// Checks if any box is selected.
#[derive(SystemParam)]
// The docs say that the lifetimes are needed (idc why)
pub struct SelectedBoxCheck<'w, 's> {
    select_check: Query<'w, 's, Entity, With<crate::input::SelectedBox>>,
}

impl<'w, 's> SelectedBoxCheck<'w, 's> {
    pub fn any_selected(&self) -> bool {
        self.select_check.is_empty()
    }
}
