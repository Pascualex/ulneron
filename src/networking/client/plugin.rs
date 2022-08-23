use std::net::UdpSocket;

use bevy::prelude::*;

use crate::networking::client::systems::*;

use super::resources::DownstreamBuffer;

#[derive(Default)]
pub struct ClientNetworkingPlugin;

impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let addr = format!("0.0.0.0:{}", fastrand::u32(49152..65535));
        let socket = UdpSocket::bind(addr).unwrap();
        socket.connect("127.0.0.1:34243").unwrap();
        socket.set_nonblocking(true).unwrap();
        app.insert_resource(socket)
            .init_resource::<DownstreamBuffer>()
            .add_system_to_stage(CoreStage::First, downstream_receiver)
            .add_system_to_stage(
                CoreStage::First,
                downstream_writer.after(downstream_receiver),
            )
            .add_system_to_stage(CoreStage::Last, upstream_reader_sender);
    }
}
