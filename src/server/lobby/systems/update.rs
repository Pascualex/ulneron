use bevy::prelude::*;

use crate::server::{
    controller::resources::ControllersInfo,
    lobby::{
        data::Lobby,
        events::LobbyDownstreamEvent,
        resources::{LobbyState, PlayersInfo},
    },
};

pub fn update(
    mut state: ResMut<LobbyState>,
    controllers_info: Res<ControllersInfo>,
    mut players_info: ResMut<PlayersInfo>,
    mut writer: EventWriter<LobbyDownstreamEvent>,
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
    writer.send(LobbyDownstreamEvent::new(lobby, locked));
}
