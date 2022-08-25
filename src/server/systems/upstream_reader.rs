use bevy::prelude::*;

use crate::{protocol::events::UpstreamEvent, server::resources::TickBuilder};

pub fn upstream_reader(
    mut upstream_reader: EventReader<UpstreamEvent>,
    mut tick_builder: ResMut<TickBuilder>,
) {
    for upstream in upstream_reader.iter() {
        let action = upstream.action.clone();
        tick_builder.tick.insert(upstream.player_id, action);
    }
}
