use bevy::prelude::*;

use crate::{networking::client::resources::DownstreamBuffer, protocol::events::DownstreamEvent};

pub fn downstream_writer(
    mut tick_buffer: ResMut<DownstreamBuffer>,
    mut downstream_writer: EventWriter<DownstreamEvent>,
) {
    if let Some(downstream) = tick_buffer.current() {
        downstream_writer.send(downstream.clone());
        tick_buffer.current += 1;
    }
}
