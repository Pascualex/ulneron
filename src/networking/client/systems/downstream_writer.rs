use bevy::prelude::*;

use crate::{networking::client::resources::DownstreamBuffer, protocol::events::DownstreamEvent};

pub fn downstream_writer(
    mut downstream_buffer: ResMut<DownstreamBuffer>,
    mut downstream_writer: EventWriter<DownstreamEvent>,
) {
    if let Some(tick) = downstream_buffer.current() {
        downstream_writer.send(DownstreamEvent::new(tick.clone()));
        downstream_buffer.current += 1;
    }
}
