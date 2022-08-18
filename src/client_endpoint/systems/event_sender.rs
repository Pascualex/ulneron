use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::upstream::InputEvent;

pub fn event_sender(mut input_reader: ResMut<Events<InputEvent>>, socket: Res<UdpSocket>) {
    for input in input_reader.drain() {
        let bytes = bincode::serialize(&input).unwrap();
        socket.send(&bytes).unwrap();
    }
}
