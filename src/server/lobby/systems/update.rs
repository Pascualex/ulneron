use bevy::prelude::*;

use crate::{
    protocol::{data::Lobby, events::LobbyDownstreamEvent},
    server::{
        controller::resources::ControllersInfo,
        lobby::resources::{LobbyState, PlayersInfo},
    },
};

pub fn update(
    mut state: ResMut<LobbyState>,
    controllers_info: Res<ControllersInfo>,
    mut players_info: ResMut<PlayersInfo>,
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
    players_info.uuids = controllers_info.vec.iter().map(|i| i.uuid).collect();
    let lobby = Lobby::new(players_info.uuids.clone());
    lobby_writer.send(LobbyDownstreamEvent::new(lobby, locked));
}
