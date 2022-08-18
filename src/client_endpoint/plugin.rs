use std::net::UdpSocket;

use bevy::prelude::*;

use crate::client_endpoint::systems::*;

#[derive(Default)]
pub struct ClientEndpointPlugin;

impl Plugin for ClientEndpointPlugin {
    fn build(&self, app: &mut App) {
        let socket = UdpSocket::bind("127.0.0.1:34255").unwrap();
        socket.connect("127.0.0.1:34254").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .add_system(event_receiver)
            .add_system(event_sender);
    }
}
