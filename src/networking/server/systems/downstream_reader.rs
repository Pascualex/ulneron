use bevy::prelude::*;

use crate::{
    networking::server::resources::DownstreamBuffer,
    protocol::{events::DownstreamEvent, messages::DownstreamMessage},
};

pub fn downstream_reader(
    mut reader: EventReader<DownstreamEvent>,
    mut buffer: ResMut<DownstreamBuffer>,
) {
    for event in reader.iter() {
        let sequence_number = buffer.messages.len() as u32;
        let msg = DownstreamMessage::new(sequence_number, event.data.clone());
        let bytes = bincode::serialize(&msg).unwrap();
        buffer.messages.push(bytes);
    }
}
