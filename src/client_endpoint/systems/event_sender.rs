use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::upstream::*;

pub fn event_sender(
    mut input_reader: EventReader<InputEvent>,
    mut join_reader: EventReader<JoinEvent>,
    socket: Res<UdpSocket>,
) {
    let mut events = Vec::new();

    events.extend(input_reader.iter().map(|e| UpstreamEvent::Input(e.clone())));
    events.extend(join_reader.iter().map(|e| UpstreamEvent::Join(e.clone())));

    if !events.is_empty() {
        let bytes = bincode::serialize(&events).unwrap();
        socket.send(&bytes).unwrap();
    }
}
