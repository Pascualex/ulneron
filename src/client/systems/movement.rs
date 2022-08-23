use bevy::prelude::*;

use crate::{
    client::{
        components::{Position, Velocity},
        resources::TickBuffer,
    },
    TIME_STEP,
};

pub fn movement(
    tick_buffer: Res<TickBuffer>,
    mut query: Query<(&mut Position, &Velocity, &mut Transform)>,
) {
    if tick_buffer.ticks.is_empty() {
        return;
    }

    for (mut position, velocity, mut transform) in query.iter_mut() {
        position.value += velocity.value * TIME_STEP;
        transform.translation = Vec3::new(position.value.y, 0.5, position.value.x);
    }
}
