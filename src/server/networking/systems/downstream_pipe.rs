use std::{io::Write, net::TcpStream};

use bevy::prelude::*;

use crate::server::{
    game::events::GameDownstreamEvent, lobby::events::LobbyDownstreamEvent,
    networking::DownstreamMessage,
};

pub fn downstream_pipe(
    mut lobby_reader: EventReader<LobbyDownstreamEvent>,
    mut game_reader: EventReader<GameDownstreamEvent>,
    mut streams: ResMut<Vec<TcpStream>>,
) {
    if lobby_reader.is_empty() && game_reader.is_empty() {
        return;
    }
    let lobby_events = lobby_reader.iter().cloned().collect();
    let game_events = game_reader.iter().cloned().collect();
    let msg = DownstreamMessage::new(lobby_events, game_events);
    let bytes = bincode::serialize(&msg).unwrap();
    for stream in streams.iter_mut() {
        stream.write_all(&bytes).ok();
    }
}
