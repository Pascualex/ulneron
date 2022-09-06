use bevy::prelude::*;

use crate::{
    client::networking::{resources::*, systems::*},
    server::{game::events::GameDownstreamEvent, lobby::events::LobbyDownstreamEvent},
    BUFFER_SIZE,
};

pub struct ClientNetworkingPlugin {
    ip: String,
}

impl ClientNetworkingPlugin {
    pub fn new(ip: String) -> Self {
        Self { ip }
    }
}

impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let address = format!("{}:34243", self.ip);
        app.insert_resource(Connection::new(address))
            .insert_resource([0_u8; BUFFER_SIZE])
            .add_event::<GameDownstreamEvent>()
            .add_event::<LobbyDownstreamEvent>()
            .add_system_to_stage(CoreStage::First, downstream_pipe)
            .add_system_to_stage(CoreStage::Last, upstream_pipe);
    }
}
