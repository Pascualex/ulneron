use bevy::prelude::*;

use crate::{
    protocol::{data::Lobby, events::LobbyDownstreamEvent},
    server::controller::resources::PlayersInfo,
};

pub fn tick(players_info: Res<PlayersInfo>, mut lobby_writer: EventWriter<LobbyDownstreamEvent>) {
    let uuids = players_info.vec.iter().map(|i| i.uuid).collect();
    let lobby = Lobby::new(uuids);
    lobby_writer.send(LobbyDownstreamEvent::new(lobby));
}
