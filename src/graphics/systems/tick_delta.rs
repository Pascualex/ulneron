use std::time::Duration;

use bevy::prelude::*;

use crate::{client::resources::TickBuffer, graphics::resources::TickDelta};

pub fn tick_delta(
    mut tick_delta: ResMut<TickDelta>,
    tick_buffer: Res<TickBuffer>,
    time: Res<Time>,
) {
    tick_delta.delta = match tick_buffer.ticks.is_empty() {
        true => tick_delta.delta + time.delta(),
        false => Duration::ZERO,
    };
}
