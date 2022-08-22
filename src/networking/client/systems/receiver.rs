use std::net::UdpSocket;

use bevy::prelude::*;

use crate::protocol::events::DownstreamEvent;

pub fn receiver(socket: Res<UdpSocket>, mut downstream_writer: EventWriter<DownstreamEvent>) {
    let mut bytes = [0; 1024];
    while socket.recv(&mut bytes).is_ok() {
        let downstream = bincode::deserialize(&bytes).unwrap();
        downstream_writer.send(downstream);
    }
}
