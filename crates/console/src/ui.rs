use crate::input::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ConsoleUI;

/// Needed to handle drag events
#[derive(Component)]
pub struct DragData(Vec2);

pub fn create_ui(mut commands: Commands) {
    commands
        .spawn((
            ConsoleUI,
            Node {
                width: px(500),
                height: px(250),
                border: UiRect::all(px(5)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderRadius::all(px(5)),
            BorderColor::all(Color::srgba(0.4, 0.4, 0.4, 0.6)),
            DragData(Vec2::ZERO),
            GlobalZIndex(i32::MAX), // forever render ontop, I worry this isn't passed down to children
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            children![message_container(), text_input_box()],
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
        Node {
            width: percent(100),
            height: px(24),
            align_self: AlignSelf::End,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    )
}

#[derive(Component, Reflect, Debug)]
pub struct MessageContainer;

pub fn message_container() -> impl Bundle {
    (
        MessageContainer,
        Node {
            width: percent(100),
            height: percent(100),
            align_self: AlignSelf::End,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        //BackgroundColor(Color::srgb(1.0, 0.2, 0.2)),
    )
}
