use std::net::UdpSocket;

use bevy::prelude::*;

use crate::networking::server::{resources::*, systems::*};

#[derive(Default)]
pub struct ServerNetworkingPlugin;

impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let socket = UdpSocket::bind("0.0.0.0:34243").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .init_resource::<Clients>()
            .add_system(receiver)
            .add_system(sender);
    }
}
