use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Clone, Serialize, Deserialize)]
pub struct ControllerEvent {
    pub id: usize,
    pub data: ControllerEventData,
}

impl ControllerEvent {
    pub fn new(id: usize, data: ControllerEventData) -> Self {
        Self { id, data }
    }

    pub fn new_local(data: ControllerEventData) -> Self {
        Self::new(0, data)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ControllerEventData {
    Join(Uuid),
    Action(Action),
}
