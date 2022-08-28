use bevy::prelude::*;

use crate::{networking::client::resources::DownstreamBuffer, protocol::events::DownstreamEvent};

pub fn downstream_writer(
    mut buffer: ResMut<DownstreamBuffer>,
    mut writer: EventWriter<DownstreamEvent>,
) {
    let mut events_sent = buffer.events_sent;
    while let Some(event) = buffer.events.remove(&events_sent) {
        writer.send(event);
        events_sent += 1;
    }
    buffer.events_sent = events_sent;
}
