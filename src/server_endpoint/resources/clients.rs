use std::net::SocketAddr;

use bevy::utils::HashSet;

use crate::events::downstream::SpawnEvent;

pub struct Clients {
    pub addresses: HashSet<SocketAddr>,
    pub spawns: Vec<SpawnEvent>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            addresses: HashSet::new(),
            spawns: Vec::new(),
        }
    }
}
