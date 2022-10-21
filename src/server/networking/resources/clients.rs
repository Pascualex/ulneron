use std::net::TcpStream;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Clients {
    pub streams: Vec<TcpStream>,
}
