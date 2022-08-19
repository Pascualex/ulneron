use std::net::SocketAddr;

use bevy::utils::HashSet;

pub struct Clients {
    pub addresses: HashSet<SocketAddr>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            addresses: HashSet::new(),
        }
    }
}
