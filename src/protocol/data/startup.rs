use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Startup {
    pub uuids: Vec<Uuid>,
}

impl Startup {
    pub fn new(uuids: Vec<Uuid>) -> Self {
        Self { uuids }
    }
}
