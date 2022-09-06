use bevy::prelude::*;

use crate::{
    client::lobby::resources::{LobbyState, PlayersInfo},
    server::lobby::events::LobbyDownstreamEvent,
};

pub fn update(
    mut reader: EventReader<LobbyDownstreamEvent>,
    mut state: ResMut<LobbyState>,
    mut players_info: ResMut<PlayersInfo>,
) {
    if !matches!(*state, LobbyState::Unlocked) {
        return;
    }
    for event in reader.iter() {
        players_info.uuids = event.lobby.uuids.clone();
        if event.lock {
            *state = LobbyState::Locked;
            return;
        }
    }
}
