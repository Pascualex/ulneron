use std::net::{TcpListener, TcpStream};

use bevy::prelude::*;

use crate::{networking::server::systems::*, BUFFER_SIZE};

pub struct ServerNetworkingPlugin;

impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let listener = TcpListener::bind("0.0.0.0:34243").unwrap();
        listener.set_nonblocking(true).unwrap();
        app.insert_resource(listener)
            .init_resource::<Vec<TcpStream>>()
            .insert_resource([0_u8; BUFFER_SIZE])
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(connection_listener)
                    .with_system(upstream_pipe.after(connection_listener)),
            )
            .add_system_to_stage(CoreStage::Last, downstream_pipe);
    }
}
