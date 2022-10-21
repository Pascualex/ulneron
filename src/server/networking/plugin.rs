use bevy::prelude::*;

use crate::{client::controller::events::ControllerUpstreamEvent, server::networking::systems::*};

use super::resources::{Clients, Endpoint};

pub struct ServerNetworkingPlugin;

impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Endpoint::new(34243))
            .init_resource::<Clients>()
            .add_event::<ControllerUpstreamEvent>()
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(connection_listener)
                    .with_system(upstream_pipe.after(connection_listener)),
            )
            .add_system_to_stage(CoreStage::Last, downstream_pipe);
    }
}
