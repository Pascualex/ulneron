use serde::{Deserialize, Serialize};

use crate::events::upstream::*;

#[derive(Serialize, Deserialize)]
pub enum UpstreamEvent {
    Input(InputEvent),
    Join(JoinEvent),
}
