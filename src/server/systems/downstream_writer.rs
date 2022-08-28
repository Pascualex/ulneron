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
    let data = match *state {
        GameState::Lobby => {
            let lobby = players_info.vec.iter().map(|i| i.uuid).collect();
            DownstreamData::Lobby(lobby)
        }
        GameState::Ready => {
            *state = GameState::Game;
            let startup = players_info.vec.iter().map(|i| i.uuid).collect();
            DownstreamData::Startup(startup)
        }
        GameState::Game => {
            let tick = players_info.vec.iter().map(|i| i.action.clone()).collect();
            DownstreamData::Tick(tick)
        }
    };
    writer.send(DownstreamEvent::new(data));
}
