use bevy::prelude::*;

use crate::{events::upstream::InputEvent, server::components::*};

pub fn movement_input(
    mut query: Query<&mut Velocity, With<Player>>,
    mut input_reader: ResMut<Events<InputEvent>>,
) {
    let mut velocity = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    for input in input_reader.drain() {
        velocity.value = input.value * 5.0;
    }
}
