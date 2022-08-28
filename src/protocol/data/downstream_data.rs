use serde::{Deserialize, Serialize};

use crate::protocol::data::{Startup, Tick};

#[derive(Clone, Serialize, Deserialize)]
pub enum DownstreamData {
    Startup(Startup),
    Tick(Tick),
}
