use bevy::prelude::*;

use crate::client::{resources::*, setup, systems::*};

#[derive(Default)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputState::new())
            .add_startup_system(setup)
            .add_system(movement.after(movement_sync))
            .add_system(movement_input)
            .add_system(movement_sync);
    }
}
