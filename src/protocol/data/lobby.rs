use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Lobby {
    pub uuids: Vec<Uuid>,
}

impl Lobby {
    pub fn new(uuids: Vec<Uuid>) -> Self {
        Self { uuids }
    }
}
