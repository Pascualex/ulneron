use serde::{Deserialize, Serialize};

use crate::protocol::data::UpstreamData;

#[derive(Serialize, Deserialize)]
pub struct UpstreamMessage {
    pub data: UpstreamData,
    pub rollback: Option<u32>,
}

impl UpstreamMessage {
    pub fn new(data: UpstreamData, rollback: Option<u32>) -> Self {
        Self { data, rollback }
    }
}
