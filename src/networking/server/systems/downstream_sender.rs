use std::net::UdpSocket;

use bevy::prelude::*;

use crate::networking::server::resources::{Clients, DownstreamBuffer};

pub fn downstream_sender(
    tick_buffer: Res<DownstreamBuffer>,
    mut clients: ResMut<Clients>,
    socket: Res<UdpSocket>,
) {
    for (address, client) in clients.map.iter_mut() {
        if (client.sequence_number as usize) < tick_buffer.messages.len() {
            let bytes = &tick_buffer.messages[client.sequence_number as usize];
            socket.send_to(bytes, address).unwrap();
            client.sequence_number += 1;
        }
    }
}
