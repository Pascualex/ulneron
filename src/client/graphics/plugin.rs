use bevy::prelude::*;

use crate::client::graphics::{resources::*, setup, systems::*};

pub struct ClientGraphicsPlugin;

#[derive(StageLabel)]
pub enum ClientGraphicsStage {
    GraphicsUpdate,
}

impl Plugin for ClientGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickDelta>()
            .add_startup_system(setup)
            .add_stage_after(
                CoreStage::Update,
                ClientGraphicsStage::GraphicsUpdate,
                SystemStage::parallel(),
            )
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new() //
                    .with_system(spawn)
                    .with_system(tick_delta),
            )
            .add_system_set_to_stage(
                ClientGraphicsStage::GraphicsUpdate,
                SystemSet::new()
                    .with_system(movement)
                    .with_system(camera.after(movement)),
            );
    }
}
