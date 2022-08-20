use std::net::UdpSocket;

use bevy::prelude::*;

use crate::server_endpoint::{resources::*, systems::*};

#[derive(Default)]
pub struct ServerEndpointPlugin;

impl Plugin for ServerEndpointPlugin {
    fn build(&self, app: &mut App) {
        let socket = UdpSocket::bind("0.0.0.0:34243").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .insert_resource(Clients::new())
            .add_system(event_receiver)
            .add_system(event_sender);
    }
}
