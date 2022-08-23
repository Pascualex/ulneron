use bevy::prelude::*;

use crate::{networking::server::resources::DownstreamBuffer, protocol::events::DownstreamEvent};

pub fn downstream_reader(
    mut downstream_reader: EventReader<DownstreamEvent>,
    mut downstream_buffer: ResMut<DownstreamBuffer>,
) {
    for downstream in downstream_reader.iter() {
        let number = downstream_buffer.events.len();
        let bytes = bincode::serialize(&(number, downstream.clone())).unwrap();
        downstream_buffer.events.push(bytes);
    }
}
