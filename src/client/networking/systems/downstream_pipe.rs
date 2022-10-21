use std::io::Read;

use bevy::prelude::*;

use crate::{
    client::networking::resources::Connection,
    server::{
        game::events::GameDownstreamEvent, lobby::events::LobbyDownstreamEvent,
        networking::DownstreamMessage,
    },
    BUFFER_SIZE,
};

pub fn downstream_pipe(
    mut connection: ResMut<Connection>,
    mut buffer: Local<Buffer>,
    mut lobby_writer: EventWriter<LobbyDownstreamEvent>,
    mut game_writer: EventWriter<GameDownstreamEvent>,
) {
    let bytes = &mut buffer.bytes;
    while let Ok(size) = connection.stream.read(bytes) {
        if size == 0 {
            break;
        }
        let msg: DownstreamMessage = bincode::deserialize(bytes).unwrap();
        for event in msg.lobby_events {
            lobby_writer.send(event);
        }
        for event in msg.game_events {
            game_writer.send(event);
        }
    }
}

pub struct Buffer {
    pub bytes: [u8; BUFFER_SIZE],
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            bytes: [0; BUFFER_SIZE],
        }
    }
}
