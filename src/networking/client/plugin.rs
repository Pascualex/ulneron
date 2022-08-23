use std::net::UdpSocket;

use bevy::prelude::*;

use crate::{networking::client::systems::*, BUFFER_SIZE};

use super::resources::DownstreamBuffer;

pub struct ClientNetworkingPlugin {
    server_address: String,
}

impl ClientNetworkingPlugin {
    pub fn new(server_address: String) -> Self {
        Self { server_address }
    }
}

impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let client_addr = format!("0.0.0.0:{}", fastrand::u32(49152..65535));
        let socket = UdpSocket::bind(client_addr).unwrap();
        let server_addr = format!("{}:34243", self.server_address);
        socket.connect(server_addr).unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .insert_resource([0_u8; BUFFER_SIZE])
            .init_resource::<DownstreamBuffer>()
            .add_system_to_stage(CoreStage::First, downstream_receiver)
            .add_system_to_stage(
                CoreStage::First,
                downstream_writer.after(downstream_receiver),
            )
            .add_system_to_stage(CoreStage::Last, upstream_reader_sender);
    }
}
