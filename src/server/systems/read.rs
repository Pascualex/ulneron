use bevy::prelude::*;

use crate::{protocol::events::UpstreamEvent, server::resources::LastInput};

pub fn read(mut upstream_reader: EventReader<UpstreamEvent>, mut last_input: ResMut<LastInput>) {
    for upstream in upstream_reader.iter() {
        last_input.direction = upstream.direction;
    }
}
