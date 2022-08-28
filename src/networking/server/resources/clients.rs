use std::net::SocketAddr;

use bevy::utils::HashMap;

#[derive(Default)]
pub struct Clients {
    pub map: HashMap<SocketAddr, Client>,
}

pub struct Client {
    pub id: usize,
    pub sequence_number: u32,
}

impl Client {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            sequence_number: 0,
        }
    }
}
