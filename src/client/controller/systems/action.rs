use bevy::prelude::*;

use crate::{
    client::controller::resources::ActionBuilder,
    protocol::events::{ControllerEventData, ControllerUpstreamEvent},
};

pub fn action_writer(
    builder: Res<ActionBuilder>,
    mut writer: EventWriter<ControllerUpstreamEvent>,
) {
    let action = builder.action.clone();
    let data = ControllerEventData::Action(action);
    writer.send(ControllerUpstreamEvent::new_local(data));
}
