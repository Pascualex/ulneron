use std::net::TcpStream;

use bevy::prelude::*;

use crate::{networking::client::systems::*, BUFFER_SIZE};

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
        let server_addr = format!("{}:34243", self.server_address);
        let stream = TcpStream::connect(server_addr).unwrap();
        stream.set_nonblocking(true).unwrap();
        app.insert_resource(stream)
            .insert_resource([0_u8; BUFFER_SIZE])
            .add_system_to_stage(CoreStage::First, downstream_pipe)
            .add_system_to_stage(CoreStage::Last, upstream_pipe);
    }
}
