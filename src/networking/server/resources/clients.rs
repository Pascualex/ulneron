use std::net::SocketAddr;

use bevy::utils::HashMap;

#[derive(Default)]
pub struct Clients {
    pub map: HashMap<SocketAddr, Client>,
}

pub struct Client {
    pub sequence_number: u32,
}

impl Client {
    pub fn new() -> Self {
        Self { sequence_number: 0 }
    }
}
