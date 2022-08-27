use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    server::{resources::*, systems::*},
    TIME_STEP,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickBuilder>()
            .init_resource::<GameState>()
            .add_system(upstream_reader)
            .add_system(game_start)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(downstream_writer.after(upstream_reader)),
            );
    }
}
