use std::io::Write;

use bevy::prelude::*;

use crate::server::{
    game::events::GameDownstreamEvent,
    lobby::events::LobbyDownstreamEvent,
    networking::{resources::Clients, DownstreamMessage},
};

pub fn downstream_pipe(
    mut lobby_reader: EventReader<LobbyDownstreamEvent>,
    mut game_reader: EventReader<GameDownstreamEvent>,
    mut clients: ResMut<Clients>,
) {
    if lobby_reader.is_empty() && game_reader.is_empty() {
        return;
    }
    let lobby_events = lobby_reader.iter().cloned().collect();
    let game_events = game_reader.iter().cloned().collect();
    let msg = DownstreamMessage::new(lobby_events, game_events);
    let bytes = bincode::serialize(&msg).unwrap();
    for stream in clients.streams.iter_mut() {
        stream.write_all(&bytes).ok();
    }
}
