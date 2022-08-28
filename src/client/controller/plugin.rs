use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    client::controller::{resources::*, systems::*},
    TIME_STEP,
};

pub struct ClientControllerPlugin;

impl Plugin for ClientControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControllerInfo>()
            .init_resource::<ActionBuilder>()
            .add_system_to_stage(CoreStage::Update, input_reader)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                upstream_writer.with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            );
    }
}
