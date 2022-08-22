use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    server::{resources::*, systems::*},
    TIME_STEP,
};

#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastInput>()
            .add_system(read)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(tick.after(read)),
            );
    }
}
