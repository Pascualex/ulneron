use std::net::UdpSocket;

use bevy::prelude::*;

use crate::protocol::{events::UpstreamEvent, messages::UpstreamMessage};

pub fn upstream_reader_sender(mut reader: EventReader<UpstreamEvent>, sender: Res<UdpSocket>) {
    if let Some(ev) = reader.iter().last() {
        let msg = UpstreamMessage::new(ev.action.clone(), None);
        let bytes = bincode::serialize(&msg).unwrap();
        sender.send(&bytes).unwrap();
    }
}
