use bevy::prelude::*;

use crate::{events::downstream::MovementEvent, server::components::*};

pub fn movement_sync(
    query: Query<(&Id, &Position, &Velocity)>,
    mut movement_writer: EventWriter<MovementEvent>,
) {
    for (id, position, velocity) in query.iter() {
        movement_writer.send(MovementEvent::new(id.value, position.value, velocity.value));
    }
}
