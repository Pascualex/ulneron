use crate::protocol::data::DownstreamData;

pub struct DownstreamEvent {
    pub data: DownstreamData,
}

impl DownstreamEvent {
    pub fn new(data: DownstreamData) -> Self {
        Self { data }
    }
}
