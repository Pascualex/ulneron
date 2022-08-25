use std::time::Duration;

use bevy::prelude::*;

use crate::{client::resources::Ticks, graphics::resources::TickDelta};

pub fn tick_delta(ticks: Res<Ticks>, time: Res<Time>, mut tick_delta: ResMut<TickDelta>) {
    tick_delta.delta = match ticks.is_empty() {
        true => tick_delta.delta + time.delta(),
        false => Duration::ZERO,
    };
}
