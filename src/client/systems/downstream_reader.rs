use bevy::prelude::*;

use crate::{client::resources::TickBuffer, protocol::events::DownstreamEvent};

pub fn downstream_reader(
    mut downstream_reader: EventReader<DownstreamEvent>,
    mut tick_buffer: ResMut<TickBuffer>,
) {
    if !tick_buffer.ticks.is_empty() {
        tick_buffer.ticks.remove(0);
    }

    for downstream in downstream_reader.iter() {
        let tick = downstream.tick.clone();
        tick_buffer.ticks.push(tick);
    }
}
