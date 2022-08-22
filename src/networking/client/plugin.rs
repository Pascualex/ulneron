use std::net::UdpSocket;

use bevy::prelude::*;

use crate::networking::client::systems::*;

#[derive(Default)]
pub struct ClientNetworkingPlugin;

impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let addr = format!("0.0.0.0:{}", fastrand::u32(49152..65535));
        let socket = UdpSocket::bind(addr).unwrap();
        socket.connect("127.0.0.1:34243").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .add_system(receiver)
            .add_system(sender);
    }
}
