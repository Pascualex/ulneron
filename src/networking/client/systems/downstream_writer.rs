use bevy::prelude::*;

use crate::{networking::client::resources::DownstreamBuffer, protocol::events::DownstreamEvent};

pub fn downstream_writer(
    mut buffer: ResMut<DownstreamBuffer>,
    mut writer: EventWriter<DownstreamEvent>,
) {
    let mut events_sent = buffer.events_sent;
    while let Some(data) = buffer.data.remove(&events_sent) {
        writer.send(DownstreamEvent::new(data));
        events_sent += 1;
    }
    buffer.events_sent = events_sent;
}
