use serde::{Deserialize, Serialize};

use crate::protocol::events::{GameEvent, LobbyEvent};

#[derive(Serialize, Deserialize)]
pub struct DownstreamMessage {
    pub lobby_events: Vec<LobbyEvent>,
    pub game_events: Vec<GameEvent>,
}

impl DownstreamMessage {
    pub fn new(lobby_events: Vec<LobbyEvent>, game_events: Vec<GameEvent>) -> Self {
        Self {
            lobby_events,
            game_events,
        }
    }
}
