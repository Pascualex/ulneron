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
            .add_system(input_reader)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(upstream_writer),
            );
    }
}
