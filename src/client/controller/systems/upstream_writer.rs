use bevy::prelude::*;

use crate::{
    client::{
        controller::resources::{ActionBuilder, ControllerInfo},
        game::resources::GameState,
    },
    protocol::{data::UpstreamData, events::UpstreamEvent},
};

pub fn upstream_writer(
    game_state: Res<GameState>,
    controller_info: Res<ControllerInfo>,
    builder: Res<ActionBuilder>,
    mut writer: EventWriter<UpstreamEvent>,
) {
    let data = match (game_state.started, controller_info.id.is_some()) {
        (false, false) => UpstreamData::Join(controller_info.uuid),
        (true, true) => UpstreamData::Action(builder.action.clone()),
        _ => return,
    };
    writer.send(UpstreamEvent::new_local(data));
}
