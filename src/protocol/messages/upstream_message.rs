use serde::{Deserialize, Serialize};

use crate::protocol::data::UpstreamData;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub data: UpstreamData,
}

impl UpstreamMessage {
    pub fn new(data: UpstreamData) -> Self {
        Self { data }
    }
}
