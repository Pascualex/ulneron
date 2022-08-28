use bevy::prelude::*;

use crate::protocol::events::*;

pub struct ProtocolPlugin;

#[derive(StageLabel)]
pub enum ProtocolStage {
    GraphicsUpdate,
}

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpstreamEvent>()
            .add_event::<DownstreamEvent>()
            .add_stage_after(
                CoreStage::Update,
                ProtocolStage::GraphicsUpdate,
                SystemStage::single_threaded(),
            );
    }
}
