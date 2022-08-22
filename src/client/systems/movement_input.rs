use bevy::prelude::*;

use crate::protocol::events::UpstreamEvent;

pub fn movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut upstream_writer: EventWriter<UpstreamEvent>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }

    if direction != Vec2::ZERO {
        upstream_writer.send(UpstreamEvent { direction });
    }
}
