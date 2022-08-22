use bevy::{prelude::*, time::FixedTimestep};
use uuid::Uuid;

use crate::{client::{setup, systems::*}, TIME_STEP};

#[derive(Default)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Uuid::new_v4())
            .init_resource::<Option<Entity>>()
            .add_startup_system(setup)
            .add_system(movement)
            .add_system(spawn)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(movement_input),
            );
    }
}
