use std::net::UdpSocket;

use bevy::prelude::*;

use crate::events::downstream::MovementEvent;

pub fn event_sender(mut movement_reader: ResMut<Events<MovementEvent>>, socket: Res<UdpSocket>) {
    for movement in movement_reader.drain() {
        let bytes = bincode::serialize(&movement).unwrap();
        socket.send(&bytes).unwrap();
    }
}
