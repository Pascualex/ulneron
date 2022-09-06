use serde::{Deserialize, Serialize};

use crate::server::{game::events::GameDownstreamEvent, lobby::events::LobbyDownstreamEvent};

#[derive(Serialize, Deserialize)]
pub struct DownstreamMessage {
    pub lobby_events: Vec<LobbyDownstreamEvent>,
    pub game_events: Vec<GameDownstreamEvent>,
}

impl DownstreamMessage {
    pub fn new(
        lobby_events: Vec<LobbyDownstreamEvent>,
        game_events: Vec<GameDownstreamEvent>,
    ) -> Self {
        Self {
            lobby_events,
            game_events,
        }
    }
}
