use bevy::prelude::*;

use crate::{
    protocol::{data::Lobby, events::LobbyEvent},
    server::controller::resources::PlayersInfo,
};

pub fn tick(players_info: Res<PlayersInfo>, mut lobby_writer: EventWriter<LobbyEvent>) {
    let uuids = players_info.vec.iter().map(|i| i.uuid).collect();
    let lobby = Lobby::new(uuids);
    lobby_writer.send(LobbyEvent::new(lobby));
}
