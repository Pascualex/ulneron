use std::net::UdpSocket;

use bevy::{prelude::*, utils::hashbrown::hash_map::Entry};

use crate::{
    networking::server::resources::{Client, Clients},
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
        let next_id = clients.map.len() as u32 + 1;
        let client = match clients.map.entry(address) {
            Entry::Occupied(client) => client.into_mut(),
            Entry::Vacant(v) => v.insert(Client::new(next_id)),
        };
        let message: UpstreamMessage = bincode::deserialize(bytes.as_ref()).unwrap();
        let event = UpstreamEvent::new(client.player_id, message.action);
        upstream_writer.send(event);
    }
}
