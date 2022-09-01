use std::{io::Write, net::TcpStream};

use bevy::prelude::*;

use crate::protocol::{events::GameEvent, messages::DownstreamMessage};

pub fn downstream_pipe(mut reader: EventReader<GameEvent>, mut streams: ResMut<Vec<TcpStream>>) {
    for event in reader.iter() {
        for stream in streams.iter_mut() {
            let msg = DownstreamMessage::Game(event.clone());
            let bytes = bincode::serialize(&msg).unwrap();
            stream.write_all(&bytes).ok();
        }
    }
}
