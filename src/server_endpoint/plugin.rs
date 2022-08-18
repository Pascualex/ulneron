use std::{net::UdpSocket, thread, time::Duration};

use bevy::prelude::*;

use crate::server_endpoint::systems::*;

#[derive(Default)]
pub struct ServerEndpointPlugin;

impl Plugin for ServerEndpointPlugin {
    fn build(&self, app: &mut App) {
        let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
        thread::sleep(Duration::from_secs(10));
        socket.connect("127.0.0.1:34255").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .add_system(event_receiver)
            .add_system(event_sender);
    }
}
