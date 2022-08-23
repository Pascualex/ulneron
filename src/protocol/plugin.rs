use bevy::prelude::*;

use crate::protocol::events::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpstreamEvent>()
            .add_event::<DownstreamEvent>();
    }
}
