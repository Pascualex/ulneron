use bevy::prelude::*;

use crate::{protocol::events::DownstreamEvent, server::resources::TickBuilder};

pub fn downstream_writer(builder: Res<TickBuilder>, mut writer: EventWriter<DownstreamEvent>) {
    let tick = builder.tick.clone();
    writer.send(DownstreamEvent::new(tick));
}
