use bevy::prelude::*;

use crate::{
    protocol::events::{UpstreamData, UpstreamEvent},
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
                if !state.ready && event.id as usize == players_info.vec.len() {
                    players_info.vec.push(PlayerInfo::new(*uuid));
                }
            }
            UpstreamData::Action(action) => {
                players_info.vec[event.id as usize].action = action.clone();
            }
        }
    }
}
