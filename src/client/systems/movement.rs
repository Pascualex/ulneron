use bevy::prelude::*;

use crate::{events::downstream::MovementEvent, client::components::Player};

pub fn movement(
    mut query: Query<&mut Transform, With<Player>>,
    mut movement_reader: ResMut<Events<MovementEvent>>,
) {
    let mut transform = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    for movement in movement_reader.drain() {
        transform.translation = Vec3::new(movement.value.y, 0.0, movement.value.x);
    }
}
