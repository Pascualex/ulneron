use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{
    networking::client::resources::DownstreamBuffer,
    protocol::{events::UpstreamEvent, messages::UpstreamMessage},
};

pub fn upstream_reader_sender(
    mut reader: EventReader<UpstreamEvent>,
    mut buffer: ResMut<DownstreamBuffer>,
    sender: Res<UdpSocket>,
) {
    if let Some(ev) = reader.iter().last() {
        let rollback = match buffer.patience == 0 {
            true => {
                buffer.patience = 5;
                Some(buffer.sequence_number)
            }
            false => None,
        };
        let msg = UpstreamMessage::new(ev.id, ev.action.clone(), rollback);
        let bytes = bincode::serialize(&msg).unwrap();
        sender.send(&bytes).unwrap();
    }
}
