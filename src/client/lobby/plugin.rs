use bevy::prelude::*;

use crate::client::lobby::{resources::*, systems::*};

pub struct ClientLobbyPlugin;

impl Plugin for ClientLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayersInfo>()
            .add_system_to_stage(CoreStage::PreUpdate, update);
    }
}
