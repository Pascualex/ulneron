use std::net::SocketAddr;

use bevy::utils::HashSet;

#[derive(Default)]
pub struct Clients {
    pub addresses: HashSet<SocketAddr>,
}
