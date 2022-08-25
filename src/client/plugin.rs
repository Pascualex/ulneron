use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    client::{resources::*, setup, systems::*},
    TIME_STEP,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Ticks>()
            .init_resource::<PlayerEntities>()
            .add_startup_system(setup)
            .add_system_to_stage(CoreStage::PreUpdate, downstream_reader)
            .add_system_to_stage(CoreStage::PreUpdate, spawn.after(downstream_reader))
            .add_system(movement)
            .add_system(movement_controller.after(movement))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(upstream_writer),
            );
    }
}
