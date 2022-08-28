use serde::{Deserialize, Serialize};

use crate::protocol::data::DownstreamData;

#[derive(Serialize, Deserialize)]
pub struct DownstreamMessage {
    pub sequence_number: u32,
    pub data: DownstreamData,
}

impl DownstreamMessage {
    pub fn new(sequence_number: u32, data: DownstreamData) -> Self {
        Self {
            sequence_number,
            data,
        }
    }
}
