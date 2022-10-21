use std::time::Duration;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct TickDelta {
    pub delta: Duration,
}
