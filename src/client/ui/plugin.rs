use bevy::prelude::*;

use crate::client::ui::{setup, systems::*};

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new().with_system(lobby).with_system(upgrades),
        );
    }
}
