use bevy::prelude::*;

use crate::client::controller::resources::ActionBuilder;

pub fn input_reader(input: Res<Input<KeyCode>>, mut builder: ResMut<ActionBuilder>) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    builder.action.direction = direction;
}
