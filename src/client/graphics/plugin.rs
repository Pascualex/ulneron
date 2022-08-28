use bevy::prelude::*;

use crate::client::graphics::{resources::*, setup, systems::*};

pub struct ClientGraphicsPlugin;

#[derive(StageLabel)]
enum ClientGraphicsStage {
    Spawn,
}

impl Plugin for ClientGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickDelta>()
            .add_stage_before(
                CoreStage::Update,
                ClientGraphicsStage::Spawn,
                SystemStage::single_threaded(),
            )
            .add_startup_system(setup)
            .add_system_to_stage(ClientGraphicsStage::Spawn, spawn)
            .add_system(tick_delta)
            .add_system(movement.after(tick_delta))
            .add_system(camera.after(movement));
    }
}
