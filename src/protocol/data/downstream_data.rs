use serde::{Deserialize, Serialize};

use crate::protocol::data::{Lobby, Startup, Tick};

#[derive(Clone, Serialize, Deserialize)]
pub enum DownstreamData {
    Lobby(Lobby),
    Startup(Startup),
    Tick(Tick),
}
