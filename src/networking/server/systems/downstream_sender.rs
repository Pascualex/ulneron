use std::net::UdpSocket;

use bevy::prelude::*;

use crate::networking::server::resources::{Clients, DownstreamBuffer};

pub fn downstream_sender(
    tick_buffer: Res<DownstreamBuffer>,
    mut clients: ResMut<Clients>,
    socket: Res<UdpSocket>,
) {
    for (address, current) in clients.map.iter_mut() {
        if *current < tick_buffer.events.len() {
            let bytes = &tick_buffer.events[*current];
            socket.send_to(bytes, address).unwrap();
            *current += 1;
        }
    }
}
