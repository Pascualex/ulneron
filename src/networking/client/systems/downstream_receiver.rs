use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{networking::client::resources::DownstreamBuffer, BUFFER_SIZE};

pub fn downstream_receiver(
    socket: Res<UdpSocket>,
    mut tick_buffer: ResMut<DownstreamBuffer>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
) {
    while socket.recv(bytes.as_mut()).is_ok() {
        let (number, downstream) = bincode::deserialize(bytes.as_ref()).unwrap();
        tick_buffer.events.insert(number, downstream);
    }
}
