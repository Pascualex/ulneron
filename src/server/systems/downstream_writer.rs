use bevy::prelude::*;

use crate::{protocol::events::DownstreamEvent, server::resources::TickBuilder};

pub fn downstream_writer(
    tick_builder: Res<TickBuilder>,
    mut downstream_writer: EventWriter<DownstreamEvent>,
) {
    let tick = tick_builder.tick.clone();
    downstream_writer.send(DownstreamEvent::new(tick));
}
