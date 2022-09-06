use bevy::prelude::*;

use crate::{
    client::game::{
        components::{Position, Velocity},
        resources::Ticks,
    },
    TICK_STEP,
};

pub fn movement(ticks: Res<Ticks>, mut query: Query<(&mut Position, &Velocity)>) {
    if ticks.current.is_none() {
        return;
    }

    for (mut position, velocity) in query.iter_mut() {
        position.value += velocity.value * TICK_STEP;
    }
}
