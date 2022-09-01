use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    server::game::{resources::*, systems::*},
    TIME_STEP,
};

pub struct ServerGamePlugin;

impl Plugin for ServerGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::Lobby)
            .add_system_to_stage(CoreStage::Update, game_start)
            .add_system_to_stage(
                CoreStage::Update,
                tick.with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            );
    }
}
