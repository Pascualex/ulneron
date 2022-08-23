use std::net::UdpSocket;

use bevy::prelude::*;

use crate::networking::client::resources::DownstreamBuffer;

pub fn downstream_receiver(socket: Res<UdpSocket>, mut tick_buffer: ResMut<DownstreamBuffer>) {
    let mut bytes = [0; 1024];
    while socket.recv(&mut bytes).is_ok() {
        let (number, downstream) = bincode::deserialize(&bytes).unwrap();
        tick_buffer.events.insert(number, downstream);
    }
}
