use std::net::SocketAddr;

use bevy::utils::HashMap;

#[derive(Default)]
pub struct Clients {
    pub map: HashMap<SocketAddr, u32>,
}
