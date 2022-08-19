use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{events::downstream::*, server_endpoint::resources::Clients};

pub fn event_sender(
    mut movement_reader: EventReader<MovementEvent>,
    mut spawn_reader: EventReader<SpawnEvent>,
    socket: Res<UdpSocket>,
    clients: Res<Clients>,
) {
    let mut events = Vec::new();

    events.extend(
        movement_reader
            .iter()
            .map(|e| DownstreamEvent::Movement(e.clone())),
    );
    events.extend(
        spawn_reader
            .iter()
            .map(|e| DownstreamEvent::Spawn(e.clone())),
    );

    if !events.is_empty() {
        let bytes = bincode::serialize(&events).unwrap();
        for client in clients.addresses.iter() {
            socket.send_to(&bytes, client).unwrap();
        }
    }
}
