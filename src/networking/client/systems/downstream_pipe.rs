use std::{io::Read, net::TcpStream};

use bevy::prelude::*;

use crate::{
    protocol::{
        events::{GameEvent, LobbyEvent},
        messages::DownstreamMessage,
    },
    BUFFER_SIZE,
};

pub fn downstream_pipe(
    mut receiver: ResMut<TcpStream>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
    mut lobby_writer: EventWriter<LobbyEvent>,
    mut game_writer: EventWriter<GameEvent>,
) {
    let bytes = bytes.as_mut();
    while let Ok(size) = receiver.read(bytes) {
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
