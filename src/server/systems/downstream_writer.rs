use bevy::prelude::*;

use crate::{
    protocol::{data::DownstreamData, events::DownstreamEvent},
    server::resources::{GameState, PlayersInfo},
};

pub fn downstream_writer(
    mut state: ResMut<GameState>,
    players_info: Res<PlayersInfo>,
    mut writer: EventWriter<DownstreamEvent>,
) {
    if matches!(*state, GameState::Ready) {
        let startup = players_info.vec.iter().map(|i| i.uuid).collect();
        let data = DownstreamData::Startup(startup);
        writer.send(DownstreamEvent::new(data));
        *state = GameState::Game;
    }
    if matches!(*state, GameState::Game) {
        let tick = players_info.vec.iter().map(|i| i.action.clone()).collect();
        let data = DownstreamData::Tick(tick);
        writer.send(DownstreamEvent::new(data));
    }
}
