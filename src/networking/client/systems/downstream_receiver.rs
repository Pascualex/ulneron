use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    networking::client::resources::DownstreamBuffer, protocol::messages::DownstreamMessage,
    BUFFER_SIZE,
};

pub fn downstream_receiver(
    socket: Res<UdpSocket>,
    mut downstream_buffer: ResMut<DownstreamBuffer>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
) {
    while socket.recv(bytes.as_mut()).is_ok() {
        let downstream: DownstreamMessage = bincode::deserialize(bytes.as_ref()).unwrap();
        let key = downstream.sequence_number;
        downstream_buffer.ticks.insert(key, downstream.tick);
    }
}
