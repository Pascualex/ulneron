use bevy::prelude::*;

use crate::{networking::client::resources::DownstreamBuffer, protocol::events::DownstreamEvent};

pub fn downstream_writer(
    mut buffer: ResMut<DownstreamBuffer>,
    mut writer: EventWriter<DownstreamEvent>,
) {
    if let Some(tick) = buffer.current() {
        writer.send(DownstreamEvent::new(tick.clone()));
        buffer.sequence_number += 1;
    }
}
