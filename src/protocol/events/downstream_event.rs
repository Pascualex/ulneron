use serde::{Deserialize, Serialize};

use crate::protocol::data::{Startup, Tick};

#[derive(Clone, Serialize, Deserialize)]
pub enum DownstreamEvent {
    Startup(Startup),
    Tick(Tick),
}
