use bevy::prelude::*;
use uuid::Uuid;

use crate::client::{resources::*, setup, systems::*};

#[derive(Default)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntitiesIds::new())
            .insert_resource(InputState::new())
            .insert_resource(Uuid::new_v4())
            .add_startup_system(setup)
            .add_system(spawn_sync)
            .add_system(movement.after(movement_sync))
            .add_system(movement_input)
            .add_system(movement_sync.after(spawn_sync));
    }
}
