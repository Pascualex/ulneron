use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::downstream::*;

pub fn event_receiver(socket: Res<UdpSocket>, mut movement_writer: EventWriter<MovementEvent>) {
    let mut bytes = [0; 1024];
    while socket.recv(&mut bytes).is_ok() {
        let events: Vec<DownstreamEvent> = bincode::deserialize(&bytes).unwrap();
        for event in events {
            match event {
                DownstreamEvent::Movement(e) => movement_writer.send(e),
            }
        }
    }
}
