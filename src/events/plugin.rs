use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    events::{cleanup, downstream::*, upstream::*},
    TIME_STEP,
};

#[derive(Default)]
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // upstream events
            .init_resource::<Events<InputEvent>>()
            .init_resource::<Events<JoinEvent>>()
            // downstream events
            .init_resource::<Events<MovementEvent>>()
            .init_resource::<Events<SpawnEvent>>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP))
                    .with_system(cleanup),
            );
    }
}
