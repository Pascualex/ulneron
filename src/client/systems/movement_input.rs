use bevy::prelude::*;
use uuid::Uuid;

use crate::{client::resources::InputState, events::upstream::InputEvent};

pub fn movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_state: ResMut<InputState>,
    id: Res<Uuid>,
    mut writer: EventWriter<InputEvent>,
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

    if direction != input_state.previous_direction {
        input_state.previous_direction = direction;
        writer.send(InputEvent::new(*id, direction));
    }
}
