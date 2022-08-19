use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::upstream::*;

pub fn event_sender(
    mut input_reader: ResMut<Events<InputEvent>>,
    mut join_reader: ResMut<Events<JoinEvent>>,
    socket: Res<UdpSocket>,
) {
    let mut events = Vec::new();

    events.extend(input_reader.drain().map(UpstreamEvent::Input));
    events.extend(join_reader.drain().map(UpstreamEvent::Join));

    if !events.is_empty() {
        let bytes = bincode::serialize(&events).unwrap();
        socket.send(&bytes).unwrap();
    }
}
