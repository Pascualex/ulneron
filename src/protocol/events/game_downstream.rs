use serde::{Deserialize, Serialize};

use crate::protocol::data::Tick;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameDownstreamEvent {
    pub tick: Tick,
}

impl GameDownstreamEvent {
    pub fn new(tick: Tick) -> Self {
        Self { tick }
    }
}
