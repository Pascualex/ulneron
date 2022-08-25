use bevy::prelude::*;

use crate::{protocol::events::UpstreamEvent, server::resources::TickBuilder};

pub fn upstream_reader(mut reader: EventReader<UpstreamEvent>, mut builder: ResMut<TickBuilder>) {
    for ev in reader.iter() {
        builder.tick.insert(ev.player_id, ev.action.clone());
    }
}
