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
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new()
                    .with_system(input_reader)
                    .with_system(join_detection),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                upstream_writer.with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            );
    }
}
