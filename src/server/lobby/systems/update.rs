use bevy::prelude::*;

use crate::{
    protocol::{data::Lobby, events::LobbyDownstreamEvent},
    server::{controller::resources::PlayersInfo, lobby::resources::LobbyState},
};

pub fn update(
    mut state: ResMut<LobbyState>,
    players_info: Res<PlayersInfo>,
    mut lobby_writer: EventWriter<LobbyDownstreamEvent>,
) {
    let locked = match *state {
        LobbyState::Unlocked => false,
        LobbyState::Locking => {
            *state = LobbyState::Locked;
            true
        }
        LobbyState::Locked => return,
    };
    let uuids = players_info.vec.iter().map(|i| i.uuid).collect();
    let lobby = Lobby::new(uuids);
    lobby_writer.send(LobbyDownstreamEvent::new(lobby, locked));
}
