use std::{io::Read, net::TcpStream};

use bevy::prelude::*;

use crate::{
    protocol::{events::UpstreamEvent, messages::UpstreamMessage},
    BUFFER_SIZE,
};

pub fn upstream_pipe(
    mut streams: ResMut<Vec<TcpStream>>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
    mut writer: EventWriter<UpstreamEvent>,
) {
    let bytes = bytes.as_mut();
    for (i, stream) in streams.iter_mut().enumerate() {
        while let Ok(size) = stream.read(bytes) {
            if size == 0 {
                break;
            }
            if let Ok(msg) = bincode::deserialize::<UpstreamMessage>(bytes) {
                writer.send(UpstreamEvent::new(i + 1, msg.data));
            }
        }
    }
}
