use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    server::lobby::{resources::*, systems::*},
    TIME_STEP,
};

pub struct ServerLobbyPlugin;

impl Plugin for ServerLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LobbyState::Unlocked)
            .add_system_to_stage(CoreStage::Update, lock)
            .add_system_to_stage(
                CoreStage::Update,
                update.with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            );
    }
}
