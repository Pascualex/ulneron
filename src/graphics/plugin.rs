use bevy::prelude::*;

use crate::graphics::{resources::*, setup, systems::*};

pub struct GraphicsPlugin;

#[derive(StageLabel)]
enum GraphicsStage {
    Spawn,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickDelta>()
            .add_stage_before(
                CoreStage::Update,
                GraphicsStage::Spawn,
                SystemStage::single_threaded(),
            )
            .add_startup_system(setup)
            .add_system_to_stage(GraphicsStage::Spawn, spawn)
            .add_system(tick_delta)
            .add_system(movement.after(tick_delta))
            .add_system(camera.after(movement));
    }
}
