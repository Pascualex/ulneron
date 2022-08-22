use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{networking::server::resources::Clients, protocol::events::DownstreamEvent};

pub fn sender(
    mut downstream_reader: EventReader<DownstreamEvent>,
    clients: Res<Clients>,
    socket: Res<UdpSocket>,
) {
    for downstream in downstream_reader.iter() {
        let bytes = bincode::serialize(&downstream).unwrap();
        for client in clients.addresses.iter() {
            socket.send_to(&bytes, client).unwrap();
        }
    }
}
