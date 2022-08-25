use bevy::prelude::*;

use crate::{
    networking::server::resources::DownstreamBuffer,
    protocol::{events::DownstreamEvent, messages::DownstreamMessage},
};

pub fn downstream_reader(
    mut downstream_reader: EventReader<DownstreamEvent>,
    mut downstream_buffer: ResMut<DownstreamBuffer>,
) {
    for event in downstream_reader.iter() {
        let sequence_number = downstream_buffer.messages.len() as u32;
        let message = DownstreamMessage::new(sequence_number, event.tick.clone());
        let bytes = bincode::serialize(&message).unwrap();
        downstream_buffer.messages.push(bytes);
    }
}
