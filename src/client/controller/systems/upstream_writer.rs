use bevy::prelude::*;

use crate::{
    client::{
        controller::resources::{ActionBuilder, ControllerInfo},
        game::resources::GameState,
    },
    protocol::{data::UpstreamData, events::UpstreamEvent},
};

pub fn upstream_writer(
    state: Res<GameState>,
    controller_info: Res<ControllerInfo>,
    builder: Res<ActionBuilder>,
    mut writer: EventWriter<UpstreamEvent>,
) {
    let data = match *state {
        GameState::Lobby => UpstreamData::Join(controller_info.uuid),
        GameState::Game => UpstreamData::Action(builder.action.clone()),
    };
    writer.send(UpstreamEvent::new_local(data));
}
