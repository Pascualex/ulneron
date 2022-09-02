use bevy::prelude::*;

use crate::{
    client::controller::resources::ControllerInfo,
    protocol::events::{ControllerEventData, ControllerUpstreamEvent},
};

pub fn setup(
    controller_info: Res<ControllerInfo>,
    mut writer: EventWriter<ControllerUpstreamEvent>,
) {
    let data = ControllerEventData::Join(controller_info.uuid);
    writer.send(ControllerUpstreamEvent::new_local(data));
}
