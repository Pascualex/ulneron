use bevy::prelude::*;

use crate::game::{components::*, plugin::TIME_STEP};

pub fn movement(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut transform = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    let mut dir = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        dir.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dir.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dir.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        dir.z -= 1.0;
    }

    transform.translation += 5.0 * TIME_STEP as f32 * dir;
}
