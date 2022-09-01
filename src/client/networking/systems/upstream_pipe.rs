use std::io::Write;

use bevy::prelude::*;

use crate::{
    client::networking::resources::Connection,
    protocol::{events::ControllerEvent, messages::UpstreamMessage},
};

pub fn upstream_pipe(mut reader: EventReader<ControllerEvent>, mut connection: ResMut<Connection>) {
    if let Some(event) = reader.iter().last() {
        let msg = UpstreamMessage::new(event.data.clone());
        let bytes = bincode::serialize(&msg).unwrap();
        connection.stream.write_all(&bytes).ok();
    }
}