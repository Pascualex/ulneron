use bevy::prelude::*;

use crate::protocol::events::*;

pub struct ProtocolPlugin;

#[derive(StageLabel)]
pub enum ProtocolStage {
    GraphicsUpdate,
}

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ControllerEvent>()
            .add_event::<LobbyEvent>()
            .add_event::<GameEvent>()
            .add_stage_after(
                CoreStage::Update,
                ProtocolStage::GraphicsUpdate,
                SystemStage::single_threaded(),
            );
    }
}
