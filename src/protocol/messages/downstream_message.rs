use serde::{Deserialize, Serialize};

use crate::protocol::data::Tick;

#[derive(Serialize, Deserialize)]
pub struct DownstreamMessage {
    pub sequence_number: u32,
    pub tick: Tick,
}

impl DownstreamMessage {
    pub fn new(sequence_number: u32, tick: Tick) -> Self {
        Self {
            sequence_number,
            tick,
        }
    }
}
