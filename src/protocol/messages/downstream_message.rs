use serde::{Deserialize, Serialize};

use crate::protocol::data::DownstreamData;

#[derive(Serialize, Deserialize)]
pub struct DownstreamMessage {
    pub data: DownstreamData,
}

impl DownstreamMessage {
    pub fn new(data: DownstreamData) -> Self {
        Self { data }
    }
}
