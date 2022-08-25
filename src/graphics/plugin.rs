use bevy::prelude::*;

use crate::graphics::{resources::*, setup, systems::*};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickDelta>()
            .add_startup_system(setup)
            .add_system(spawn)
            .add_system(tick_delta)
            .add_system_to_stage(CoreStage::PostUpdate, movement);
    }
}
