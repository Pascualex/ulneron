use serde::{Deserialize, Serialize};

use crate::protocol::data::{Startup, Tick};

#[derive(Clone, Serialize, Deserialize)]
pub enum GameDownstreamEvent {
    Startup(Startup),
    Tick(Tick),
}
