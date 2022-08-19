use serde::{Deserialize, Serialize};

use crate::events::downstream::*;

#[derive(Serialize, Deserialize)]
pub enum DownstreamEvent {
    Movement(MovementEvent),
    Spawn(SpawnEvent),
}
