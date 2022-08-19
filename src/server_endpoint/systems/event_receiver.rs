use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::upstream::{InputEvent, JoinEvent, UpstreamEvent};

pub fn event_receiver(
    socket: Res<UdpSocket>,
    mut input_writer: EventWriter<InputEvent>,
    mut join_writer: EventWriter<JoinEvent>,
) {
    let mut bytes = [0; 1024];
    while socket.recv(&mut bytes).is_ok() {
        let events: Vec<UpstreamEvent> = bincode::deserialize(&bytes).unwrap();
        for event in events {
            match event {
                UpstreamEvent::Input(e) => input_writer.send(e),
                UpstreamEvent::Join(e) => join_writer.send(e),
            }
        }
    }
}
