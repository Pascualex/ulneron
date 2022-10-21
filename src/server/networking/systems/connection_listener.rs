use bevy::prelude::*;

use crate::server::networking::resources::{Clients, Endpoint};

pub fn connection_listener(endpoint: Res<Endpoint>, mut clients: ResMut<Clients>) {
    while let Ok((stream, _)) = endpoint.listener.accept() {
        stream.set_nonblocking(true).unwrap();
        clients.streams.push(stream);
    }
}
