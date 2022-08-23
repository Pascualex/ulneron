use serde::{Deserialize, Serialize};

use crate::protocol::data::Tick;

#[derive(Clone, Serialize, Deserialize)]
pub struct DownstreamEvent {
    pub tick: Tick,
}

impl DownstreamEvent {
    pub fn new(tick: Tick) -> Self {
        Self { tick }
    }
}
