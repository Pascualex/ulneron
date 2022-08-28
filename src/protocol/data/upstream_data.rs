use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::data::Action;

#[derive(Clone, Serialize, Deserialize)]
pub enum UpstreamData {
    Join(Uuid),
    Action(Action),
}
