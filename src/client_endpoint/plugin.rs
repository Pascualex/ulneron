use std::net::UdpSocket;

use bevy::prelude::*;

use crate::client_endpoint::systems::*;

#[derive(Default)]
pub struct ClientEndpointPlugin;

impl Plugin for ClientEndpointPlugin {
    fn build(&self, app: &mut App) {
        let addr = format!("0.0.0.0:{}", fastrand::u32(49152..65535));
        let socket = UdpSocket::bind(addr).unwrap();
        socket.connect("127.0.0.1:34243").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .add_system(event_receiver)
            .add_system(event_sender);
    }
}
