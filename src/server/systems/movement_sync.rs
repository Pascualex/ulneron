use bevy::prelude::*;

use crate::{events::downstream::MovementEvent, server::components::*};

pub fn movement_sync(
    query: Query<(&Position, &Velocity)>,
    mut movement_writer: EventWriter<MovementEvent>,
) {
    for (position, velocity) in query.iter() {
        movement_writer.send(MovementEvent {
            position: position.value,
            velocity: velocity.value,
        });
    }
}
