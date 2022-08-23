use bevy::utils::HashMap;

use crate::protocol::events::DownstreamEvent;

#[derive(Default)]
pub struct DownstreamBuffer {
    pub current: usize,
    pub events: HashMap<usize, DownstreamEvent>,
}

impl DownstreamBuffer {
    pub fn current(&self) -> Option<&DownstreamEvent> {
        self.events.get(&self.current)
    }
}
