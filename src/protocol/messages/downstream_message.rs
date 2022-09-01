use serde::{Deserialize, Serialize};

use crate::protocol::events::{GameEvent, LobbyEvent};

#[derive(Serialize, Deserialize)]
pub enum DownstreamMessage {
    Lobby(LobbyEvent),
    Game(GameEvent),
}
