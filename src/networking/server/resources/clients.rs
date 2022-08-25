use std::net::SocketAddr;

use bevy::utils::HashMap;

#[derive(Default)]
pub struct Clients {
    pub map: HashMap<SocketAddr, Client>,
}

pub struct Client {
    pub player_id: u32,
    pub sequence_number: u32,
}

impl Client {
    pub fn new(player_id: u32) -> Self {
        Self {
            player_id,
            sequence_number: 0,
        }
    }
}
