use bevy::{prelude::*, time::FixedTimestep};
use uuid::Uuid;

use crate::{
    client::{resources::*, setup, systems::*},
    TIME_STEP,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Uuid::new_v4())
            .init_resource::<TickBuffer>()
            .init_resource::<PlayerIds>()
            .add_startup_system(setup)
            .add_system_to_stage(CoreStage::PreUpdate, downstream_reader)
            .add_system_to_stage(CoreStage::PreUpdate, spawn.after(downstream_reader))
            .add_system(movement_view)
            .add_system(movement.after(movement_view))
            .add_system(movement_input.after(movement))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(upstream_writer),
            );
    }
}
