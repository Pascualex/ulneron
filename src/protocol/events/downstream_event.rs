use crate::protocol::data::Tick;

pub struct DownstreamEvent {
    pub tick: Tick,
}

impl DownstreamEvent {
    pub fn new(tick: Tick) -> Self {
        Self { tick }
    }
}
