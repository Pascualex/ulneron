use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::downstream::*;

pub fn event_sender(
    mut movement_reader: EventReader<MovementEvent>,
    mut spawn_reader: EventReader<SpawnEvent>,
    socket: Res<UdpSocket>,
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
        socket.send(&bytes).unwrap();
    }
}
