use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    client::controller::{events::*, resources::*, setup, systems::*},
    TICK_STEP,
};

pub struct ClientControllerPlugin;

impl Plugin for ClientControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControllerInfo>()
            .init_resource::<ActionBuilder>()
            .add_event::<ControllerUpstreamEvent>()
            .add_startup_system(setup)
            .add_system_to_stage(CoreStage::Update, input)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                action_writer.with_run_criteria(FixedTimestep::step(TICK_STEP as f64)),
            );
    }
}
