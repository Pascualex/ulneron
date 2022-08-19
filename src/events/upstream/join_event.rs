use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct JoinEvent {
    pub id: Uuid,
}

impl JoinEvent {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
