use bevy::prelude::*;

use crate::{protocol::events::DownstreamEvent, server::resources::LastInput};

pub fn tick(
    mut last_input: ResMut<LastInput>,
    mut downstream_writer: EventWriter<DownstreamEvent>,
) {
    downstream_writer.send(DownstreamEvent {
        direction: last_input.direction,
    });
    last_input.direction = Vec2::ZERO;
}
