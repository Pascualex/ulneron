use std::net::UdpSocket;

use bevy::prelude::*;

use crate::networking::server::resources::{Clients, DownstreamBuffer};

pub fn downstream_sender(
    buffer: Res<DownstreamBuffer>,
    mut clients: ResMut<Clients>,
    sender: Res<UdpSocket>,
) {
    for (address, client) in clients.map.iter_mut() {
        if (client.sequence_number as usize) < buffer.messages.len() {
            let bytes = &buffer.messages[client.sequence_number as usize];
            sender.send_to(bytes, address).unwrap();
            client.sequence_number += 1;
        }
    }
}
