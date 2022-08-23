use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    networking::server::resources::{Client, Clients},
    protocol::events::UpstreamEvent,
};

pub fn upstream_receiver_writer(
    socket: Res<UdpSocket>,
    mut clients: ResMut<Clients>,
    mut upstream_writer: EventWriter<UpstreamEvent>,
) {
    let mut bytes = [0; 1024];
    while let Ok((_, address)) = socket.recv_from(&mut bytes) {
        if !clients.map.contains_key(&address) {
            clients.map.insert(address, Client::new());
        }
        let input = bincode::deserialize(&bytes).unwrap();
        upstream_writer.send(input);
    }
}
