use bevy::prelude::*;

use crate::{
    protocol::{data::UpstreamData, events::UpstreamEvent},
    server::resources::PlayerInfo,
    server::resources::{GameState, PlayersInfo},
};

pub fn upstream_reader(
    mut reader: EventReader<UpstreamEvent>,
    state: Res<GameState>,
    mut players_info: ResMut<PlayersInfo>,
) {
    for event in reader.iter() {
        match &event.data {
            UpstreamData::Join(uuid) => {
                if matches!(*state, GameState::Lobby) && event.id == players_info.vec.len() {
                    players_info.vec.push(PlayerInfo::new(*uuid));
                }
            }
            UpstreamData::Action(action) => {
                if event.id < players_info.vec.len() {
                    players_info.vec[event.id].action = action.clone();
                }
            }
        }
    }
}
