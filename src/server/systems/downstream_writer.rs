use bevy::prelude::*;

use crate::{
    protocol::events::DownstreamEvent,
    server::resources::{GameState, TickBuilder},
};

pub fn downstream_writer(
    builder: Res<TickBuilder>,
    state: Res<GameState>,
    mut writer: EventWriter<DownstreamEvent>,
) {
    if !state.started {
        return;
    }
    let tick = builder.tick.clone();
    writer.send(DownstreamEvent::new(tick));
}
