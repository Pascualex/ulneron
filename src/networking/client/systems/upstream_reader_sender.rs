use std::net::UdpSocket;

use bevy::prelude::*;

use crate::protocol::{events::UpstreamEvent, messages::UpstreamMessage};

pub fn upstream_reader_sender(
    mut upstream_reader: EventReader<UpstreamEvent>,
    socket: Res<UdpSocket>,
) {
    if let Some(event) = upstream_reader.iter().last() {
        let message = UpstreamMessage::new(event.id, event.action.clone());
        let bytes = bincode::serialize(&message).unwrap();
        socket.send(&bytes).unwrap();
    }
}
