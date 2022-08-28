use crate::protocol::data::UpstreamData;

pub struct UpstreamEvent {
    pub id: usize,
    pub data: UpstreamData,
}

impl UpstreamEvent {
    pub fn new(id: usize, data: UpstreamData) -> Self {
        Self { id, data }
    }

    pub fn new_local(data: UpstreamData) -> Self {
        Self::new(0, data)
    }
}
