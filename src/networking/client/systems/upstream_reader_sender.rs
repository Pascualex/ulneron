use std::net::UdpSocket;

use bevy::prelude::*;

use crate::protocol::events::UpstreamEvent;

pub fn upstream_reader_sender(
    mut upstream_reader: EventReader<UpstreamEvent>,
    socket: Res<UdpSocket>,
) {
    if let Some(upstream) = upstream_reader.iter().last() {
        let bytes = bincode::serialize(upstream).unwrap();
        socket.send(&bytes).unwrap();
    }
}
