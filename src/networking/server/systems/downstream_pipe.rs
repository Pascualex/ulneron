use std::{io::Write, net::TcpStream};

use bevy::prelude::*;

use crate::protocol::{events::DownstreamEvent, messages::DownstreamMessage};

pub fn downstream_pipe(
    mut reader: EventReader<DownstreamEvent>,
    mut streams: ResMut<Vec<TcpStream>>,
) {
    for event in reader.iter() {
        for stream in streams.iter_mut() {
            let msg = DownstreamMessage::new(event.data.clone());
            let bytes = bincode::serialize(&msg).unwrap();
            stream.write_all(&bytes).ok();
        }
    }
}
