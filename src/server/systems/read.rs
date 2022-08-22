use bevy::prelude::*;

use crate::{protocol::events::UpstreamEvent, server::resources::Buffer};

pub fn read(mut upstream_reader: EventReader<UpstreamEvent>, mut buffer: ResMut<Buffer>) {
    for upstream in upstream_reader.iter() {
        buffer.directions.insert(upstream.id, upstream.direction);
    }
}
