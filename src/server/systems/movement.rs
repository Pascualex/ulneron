use bevy::prelude::*;

use crate::{server::components::*, TIME_STEP};

pub fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.value += velocity.value * TIME_STEP as f32;
    }
}
