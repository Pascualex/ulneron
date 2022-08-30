use std::{io::Read, net::TcpStream};

use bevy::prelude::*;

use crate::{
    protocol::{events::DownstreamEvent, messages::DownstreamMessage},
    BUFFER_SIZE,
};

pub fn downstream_pipe(
    mut receiver: ResMut<TcpStream>,
    mut bytes: ResMut<[u8; BUFFER_SIZE]>,
    mut writer: EventWriter<DownstreamEvent>,
) {
    let bytes = bytes.as_mut();
    while let Ok(size) = receiver.read(bytes) {
        if size == 0 {
            break;
        }
        let msg: DownstreamMessage = bincode::deserialize(bytes).unwrap();
        writer.send(DownstreamEvent::new(msg.data));
    }
}
