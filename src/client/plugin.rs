use bevy::{prelude::*, time::FixedTimestep};

use crate::client::{setup, systems::*};

pub const TIME_STEP: f64 = 1.0 / 60.0;

#[derive(Default)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(movement)
                .with_system(movement_input),
        );
    }
}
