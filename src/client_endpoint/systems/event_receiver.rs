use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::downstream::MovementEvent;

pub fn event_receiver(socket: Res<UdpSocket>, mut movement_writer: EventWriter<MovementEvent>) {
    let mut bytes = [0; 1024];
    while socket.recv(&mut bytes).is_ok() {
        let movement = bincode::deserialize(&bytes).unwrap();
        movement_writer.send(movement);
    }
}
