use serde::{Deserialize, Serialize};

use crate::protocol::data::Lobby;

#[derive(Clone, Serialize, Deserialize)]
pub struct LobbyEvent {
    pub lobby: Lobby,
}

impl LobbyEvent {
    pub fn new(lobby: Lobby) -> Self {
        Self { lobby }
    }
}
