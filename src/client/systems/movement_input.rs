use bevy::prelude::*;

use crate::events::upstream::InputEvent;

pub fn movement_input(keyboard_input: Res<Input<KeyCode>>, mut writer: EventWriter<InputEvent>) {
    let mut dir = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }

    writer.send(InputEvent { value: dir });
}
