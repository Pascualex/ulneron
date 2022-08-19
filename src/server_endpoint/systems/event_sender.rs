use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::downstream::*;

pub fn event_sender(
    mut movement_reader: ResMut<Events<MovementEvent>>,
    mut spawn_reader: ResMut<Events<SpawnEvent>>,
    socket: Res<UdpSocket>,
) {
    let mut events = Vec::new();

    events.extend(movement_reader.drain().map(DownstreamEvent::Movement));
    events.extend(spawn_reader.drain().map(DownstreamEvent::Spawn));

    if !events.is_empty() {
        let bytes = bincode::serialize(&events).unwrap();
        socket.send(&bytes).unwrap();
    }
}
