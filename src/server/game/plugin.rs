use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    server::game::{events::*, systems::*},
    TICK_STEP,
};

pub struct ServerGamePlugin;

impl Plugin for ServerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameDownstreamEvent>().add_system_to_stage(
            CoreStage::Update,
            tick.with_run_criteria(FixedTimestep::step(TICK_STEP as f64)),
        );
    }
}
