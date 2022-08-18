use bevy::{prelude::*, time::FixedTimestep};

use crate::game::{setup_system, systems::*};

pub const TIME_STEP: f64 = 1.0 / 60.0;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(movement),
        );
    }
}
