use bevy::prelude::*;

use crate::client::ui::{setup, systems::*};

pub struct ClientLobbyPlugin;

impl Plugin for ClientLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::Update, lobby);
    }
}
