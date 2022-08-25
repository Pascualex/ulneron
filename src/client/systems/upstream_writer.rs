use bevy::prelude::*;

use crate::protocol::{data::Action, events::UpstreamEvent};

pub fn upstream_writer(
    keyboard_input: Res<Input<KeyCode>>,
    mut upstream_writer: EventWriter<UpstreamEvent>,
) {
    let mut action = Action::new();

    if keyboard_input.pressed(KeyCode::Up) {
        action.direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        action.direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        action.direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        action.direction.x -= 1.0;
    }

    upstream_writer.send(UpstreamEvent::new_local(action));
}
