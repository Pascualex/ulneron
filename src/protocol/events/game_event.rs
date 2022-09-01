use serde::{Deserialize, Serialize};

use crate::protocol::data::{Startup, Tick};

#[derive(Clone, Serialize, Deserialize)]
pub enum GameEvent {
    Startup(Startup),
    Tick(Tick),
}
