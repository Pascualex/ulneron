use std::net::SocketAddr;

use bevy::utils::HashMap;

#[derive(Default)]
pub struct Clients {
    pub map: HashMap<SocketAddr, Client>,
}

#[derive(Default)]
pub struct Client {
    pub current: usize,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }
}
