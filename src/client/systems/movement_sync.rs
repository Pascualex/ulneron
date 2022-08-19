use bevy::prelude::*;

use crate::{
    client::components::{Player, Velocity},
    events::downstream::MovementEvent,
};

pub fn movement_sync(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut movement_reader: ResMut<Events<MovementEvent>>,
) {
    let (mut transform, mut velocity) = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    for movement in movement_reader.drain() {
        transform.translation = Vec3::new(movement.position.y, 0.5, movement.position.x);
        velocity.value = movement.velocity;
    }
}
