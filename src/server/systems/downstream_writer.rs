use bevy::prelude::*;

use crate::{
    protocol::events::DownstreamEvent,
    server::resources::{GameState, PlayersInfo},
};

pub fn downstream_writer(
    mut state: ResMut<GameState>,
    players_info: Res<PlayersInfo>,
    mut writer: EventWriter<DownstreamEvent>,
) {
    if !state.ready {
        return;
    }
    if !state.started {
        let startup = players_info.vec.iter().map(|i| i.uuid).collect();
        writer.send(DownstreamEvent::Startup(startup));
        state.started = true;
    }
    let tick = players_info.vec.iter().map(|i| i.action.clone()).collect();
    writer.send(DownstreamEvent::Tick(tick));
}
