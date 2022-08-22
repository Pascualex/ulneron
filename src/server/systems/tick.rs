use bevy::prelude::*;

use crate::{protocol::events::DownstreamEvent, server::resources::Buffer};

pub fn tick(mut buffer: ResMut<Buffer>, mut downstream_writer: EventWriter<DownstreamEvent>) {
    downstream_writer.send(DownstreamEvent::new(buffer.directions.clone()));
    buffer.directions.clear();
}
