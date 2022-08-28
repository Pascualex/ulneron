use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Position, Velocity},
        resources::Ticks,
    },
    TIME_STEP,
};

pub fn movement(ticks: Res<Ticks>, mut query: Query<(&mut Position, &Velocity)>) {
    if ticks.is_empty() {
        return;
    }

    for (mut position, velocity) in query.iter_mut() {
        position.value += velocity.value * TIME_STEP;
    }
}
