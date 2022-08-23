use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{networking::server::resources::Clients, protocol::events::UpstreamEvent, BUFFER_SIZE};

pub fn upstream_receiver_writer(
    socket: Res<UdpSocket>,
    mut clients: ResMut<Clients>,
    mut upstream_writer: EventWriter<UpstreamEvent>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
) {
    while let Ok((_, address)) = socket.recv_from(bytes.as_mut()) {
        clients.map.try_insert(address, 0).ok();
        let input = bincode::deserialize(bytes.as_ref()).unwrap();
        upstream_writer.send(input);
    }
}
