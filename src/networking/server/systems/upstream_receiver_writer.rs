use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    networking::server::resources::Clients,
    protocol::{events::UpstreamEvent, messages::UpstreamMessage},
    BUFFER_SIZE,
};

pub fn upstream_receiver_writer(
    socket: Res<UdpSocket>,
    mut clients: ResMut<Clients>,
    mut upstream_writer: EventWriter<UpstreamEvent>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
) {
    while let Ok((_, address)) = socket.recv_from(bytes.as_mut()) {
        clients.map.try_insert(address, 0).ok();
        let message: UpstreamMessage = bincode::deserialize(bytes.as_ref()).unwrap();
        let event = UpstreamEvent::new(message.id, message.action);
        upstream_writer.send(event);
    }
}
