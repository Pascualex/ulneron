use bevy::{prelude::*, time::FixedTimestep};

use crate::{server::lobby::systems::*, TIME_STEP};

pub struct ServerLobbyPlugin;

impl Plugin for ServerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::Update,
            tick.with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
        );
    }
}
