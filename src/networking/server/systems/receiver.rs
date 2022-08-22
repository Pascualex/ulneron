use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{networking::server::resources::Clients, protocol::events::UpstreamEvent};

pub fn receiver(
    socket: Res<UdpSocket>,
    mut clients: ResMut<Clients>,
    mut upstream_writer: EventWriter<UpstreamEvent>,
) {
    let mut bytes = [0; 1024];
    while let Ok((_, address)) = socket.recv_from(&mut bytes) {
        clients.addresses.insert(address);
        let input = bincode::deserialize(&bytes).unwrap();
        upstream_writer.send(input);
    }
}
