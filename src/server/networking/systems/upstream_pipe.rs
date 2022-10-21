use std::io::Read;

use bevy::prelude::*;

use crate::{
    client::{controller::events::ControllerUpstreamEvent, networking::UpstreamMessage},
    server::networking::resources::Clients,
    BUFFER_SIZE,
};

pub fn upstream_pipe(
    mut clients: ResMut<Clients>,
    mut buffer: Local<Buffer>,
    mut writer: EventWriter<ControllerUpstreamEvent>,
) {
    let bytes = &mut buffer.bytes;
    for (i, stream) in clients.streams.iter_mut().enumerate() {
        while let Ok(size) = stream.read(bytes) {
            if size == 0 {
                break;
            }
            if let Ok(msg) = bincode::deserialize::<UpstreamMessage>(bytes) {
                writer.send(ControllerUpstreamEvent::new(i + 1, msg.data));
            }
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
