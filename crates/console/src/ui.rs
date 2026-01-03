use crate::input::*;
use bevy::prelude::*;

pub const CONSOLE_FONT_SIZE: f32 = 12.0;

#[derive(Component)]
pub struct ConsoleUI;

/// Needed to handle drag events
/// This could be branched off into a separate crate/component
#[derive(Component)]
pub struct DragData(Vec2);

pub fn create_ui(mut commands: Commands) {
    commands
        .spawn((
            ConsoleUI,
            Node {
                width: px(750),
                height: px(250),
                border: UiRect::all(px(2)),
                flex_direction: FlexDirection::ColumnReverse,
                overflow: Overflow::clip_y(),
                ..default()
            },
            BorderRadius::all(px(5)),
            BorderColor::all(Color::BLACK.with_alpha(0.5)),
            DragData(Vec2::ZERO),
            GlobalZIndex(i32::MAX), // forever render ontop, I worry this isn't passed down to children
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            children![text_input_box(), message_container(),],
        ))
        .observe(
            |on_drag_start: On<Pointer<DragStart>>,
             mut drag_query: Query<&mut DragData>,
             transform_query: Query<&UiTransform>| {
                if let Ok(mut drag_data) = drag_query.get_mut(on_drag_start.event_target()) {
                    if let Ok(transform) = transform_query.get(on_drag_start.entity) {
                        let current_x = match transform.translation.x {
                            Val::Px(x) => x,
                            _ => 0.0,
                        };
                        let current_y = match transform.translation.y {
                            Val::Px(y) => y,
                            _ => 0.0,
                        };
                        // Store offset: pointer position minus element position
                        drag_data.0 = on_drag_start.pointer_location.position
                            - Vec2::new(current_x, current_y);
                    }
                }
            },
        )
        .observe(
            |on_drag: On<Pointer<Drag>>,
             mut query: Query<&mut UiTransform>,
             drag_query: Query<&DragData>| {
                if let Ok(mut transform) = query.get_mut(on_drag.event_target()) {
                    let offset = drag_query.get(on_drag.entity).unwrap();
                    // Subtract offset from pointer location
                    let new_pos = on_drag.pointer_location.position - offset.0;
                    transform.translation = Val2::px(new_pos.x, new_pos.y);
                }
            },
        );
}

pub fn text_input_box() -> impl Bundle {
    (
        TextInputBox::default(),
        TextFont {
            font_size: CONSOLE_FONT_SIZE,
            ..default()
        },
        Node {
            width: percent(100),
            flex_grow: 0.0,   // Don't expand
            flex_shrink: 0.0, // Don't shrink
            flex_basis: px(CONSOLE_FONT_SIZE + 2.0),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.38)),
    )
}

#[derive(Component, Reflect, Debug)]
pub struct MessageContainer;

pub fn message_container() -> impl Bundle {
    (
        MessageContainer,
        Node {
            width: percent(100),
            flex_grow: 1.0, // Expand to fill space
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip_y(),
            ..default()
        },
    )
}
