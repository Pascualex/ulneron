use bevy::prelude::*;

use crate::client::game::{resources::*, systems::*};

pub struct ClientGamePlugin;

impl Plugin for ClientGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::Lobby)
            .init_resource::<PlayersInfo>()
            .init_resource::<Ticks>()
            .add_system_to_stage(CoreStage::PreUpdate, downstream_reader)
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new()
                    .with_system(movement)
                    .with_system(movement_action.after(movement)),
            );
    }
}
