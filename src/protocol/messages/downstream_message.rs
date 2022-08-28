use serde::{Deserialize, Serialize};

use crate::protocol::events::DownstreamEvent;

#[derive(Serialize, Deserialize)]
pub struct DownstreamMessage {
    pub sequence_number: u32,
    pub event: DownstreamEvent,
}

impl DownstreamMessage {
    pub fn new(sequence_number: u32, event: DownstreamEvent) -> Self {
        Self {
            sequence_number,
            event,
        }
    }
}
