use bevy::prelude::*;

use crate::{
    client::{
        components::{Position, Velocity},
        resources::Ticks,
    },
    TIME_STEP,
};

pub fn movement(ticks: Res<Ticks>, mut query: Query<(&mut Position, &Velocity, &mut Transform)>) {
    if ticks.is_empty() {
        return;
    }

    for (mut position, velocity, mut transform) in query.iter_mut() {
        position.value += velocity.value * TIME_STEP;
        transform.translation = Vec3::new(position.value.y, 0.5, position.value.x);
    }
}
