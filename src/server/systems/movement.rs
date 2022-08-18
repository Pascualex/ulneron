use bevy::prelude::*;

use crate::{
    events::{downstream::MovementEvent, upstream::InputEvent},
    server::{components::*, plugin::TIME_STEP},
};

pub fn movement(
    mut query: Query<&mut Position, With<Player>>,
    mut input_reader: ResMut<Events<InputEvent>>,
    mut movement_writer: EventWriter<MovementEvent>,
) {
    let mut position = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    for input in input_reader.drain() {
        position.value += 5.0 * TIME_STEP as f32 * input.value;
        movement_writer.send(MovementEvent {
            value: position.value,
        });
        println!("{}", position.value);
    }
}
