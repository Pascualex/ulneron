use serde::{Deserialize, Serialize};

use crate::protocol::data::Lobby;

#[derive(Clone, Serialize, Deserialize)]
pub struct LobbyDownstreamEvent {
    pub lobby: Lobby,
}

impl LobbyDownstreamEvent {
    pub fn new(lobby: Lobby) -> Self {
        Self { lobby }
    }
}
