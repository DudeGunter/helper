use bevy::prelude::*;

pub fn quit(_: In<String>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
