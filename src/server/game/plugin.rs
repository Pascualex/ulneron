use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    server::game::{events::*, systems::*},
    TIME_STEP,
};

pub struct ServerGamePlugin;

impl Plugin for ServerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameDownstreamEvent>().add_system_to_stage(
            CoreStage::Update,
            tick.with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
        );
    }
}
