use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::upstream::InputEvent;

pub fn event_receiver(socket: Res<UdpSocket>, mut input_writer: EventWriter<InputEvent>) {
    let mut bytes = [0; 1024];
    while socket.recv(&mut bytes).is_ok() {
        let input = bincode::deserialize(&bytes).unwrap();
        input_writer.send(input);
    }
}
