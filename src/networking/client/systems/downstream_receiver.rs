use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    networking::client::resources::DownstreamBuffer, protocol::messages::DownstreamMessage,
    BUFFER_SIZE,
};

pub fn downstream_receiver(
    receiver: Res<UdpSocket>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
    mut buffer: ResMut<DownstreamBuffer>,
) {
    let bytes = bytes.as_mut();
    while receiver.recv(bytes).is_ok() {
        let msg: DownstreamMessage = bincode::deserialize(bytes).unwrap();
        buffer.patience = match msg.sequence_number == buffer.sequence_number {
            true => 5,
            false => buffer.patience.saturating_sub(1),
        };
        buffer.events.insert(msg.sequence_number, msg.event);
        while buffer.events.contains_key(&buffer.sequence_number) {
            buffer.sequence_number += 1;
        }
    }
}
