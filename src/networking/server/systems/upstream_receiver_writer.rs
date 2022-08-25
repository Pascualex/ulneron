use std::net::UdpSocket;

use bevy::{prelude::*, utils::hashbrown::hash_map::Entry};

use crate::{
    networking::server::resources::{Client, Clients},
    protocol::{events::UpstreamEvent, messages::UpstreamMessage},
    BUFFER_SIZE,
};

pub fn upstream_receiver_writer(
    receiver: Res<UdpSocket>,
    mut clients: ResMut<Clients>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
    mut writer: EventWriter<UpstreamEvent>,
) {
    let bytes = bytes.as_mut();
    while let Ok((_, address)) = receiver.recv_from(bytes) {
        let next_id = clients.map.len() as u32 + 1;
        let client = match clients.map.entry(address) {
            Entry::Occupied(client) => client.into_mut(),
            Entry::Vacant(v) => v.insert(Client::new(next_id)),
        };

        let msg: UpstreamMessage = bincode::deserialize(bytes).unwrap();

        if let Some(rollback) = msg.rollback {
            client.sequence_number = rollback;
        }

        let ev = UpstreamEvent::new(client.player_id, msg.action);
        writer.send(ev);
    }
}
