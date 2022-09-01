use std::io::Read;

use bevy::prelude::*;

use crate::{
    client::networking::resources::Connection,
    protocol::{
        events::{GameEvent, LobbyEvent},
        messages::DownstreamMessage,
    },
    BUFFER_SIZE,
};

pub fn downstream_pipe(
    mut connection: ResMut<Connection>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
    mut lobby_writer: EventWriter<LobbyEvent>,
    mut game_writer: EventWriter<GameEvent>,
) {
    let bytes = bytes.as_mut();
    while let Ok(size) = connection.stream.read(bytes) {
        if size == 0 {
            break;
        }
        let msg: DownstreamMessage = bincode::deserialize(bytes).unwrap();
        match msg {
            DownstreamMessage::Lobby(ev) => lobby_writer.send(ev),
            DownstreamMessage::Game(ev) => game_writer.send(ev),
        }
    }
}
