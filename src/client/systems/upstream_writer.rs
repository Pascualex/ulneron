use bevy::prelude::*;

use crate::protocol::{data::Action, events::UpstreamEvent};

pub fn upstream_writer(input: Res<Input<KeyCode>>, mut writer: EventWriter<UpstreamEvent>) {
    let mut action = Action::new();

    if input.pressed(KeyCode::Up) {
        action.direction.y += 1.0;
    }
    if input.pressed(KeyCode::Right) {
        action.direction.x += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        action.direction.y -= 1.0;
    }
    if input.pressed(KeyCode::Left) {
        action.direction.x -= 1.0;
    }

    writer.send(UpstreamEvent::new_local(action));
}
