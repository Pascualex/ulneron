use bevy::prelude::*;

use crate::client::ui::{setup, systems::*};

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::Update, lobby);
    }
}
