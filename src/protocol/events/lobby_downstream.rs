use serde::{Deserialize, Serialize};

use crate::protocol::data::Lobby;

#[derive(Clone, Serialize, Deserialize)]
pub struct LobbyDownstreamEvent {
    pub lobby: Lobby,
    pub lock: bool,
}

impl LobbyDownstreamEvent {
    pub fn new(lobby: Lobby, locked: bool) -> Self {
        Self {
            lobby,
            lock: locked,
        }
    }
}
